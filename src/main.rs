use std::{
    collections::HashSet,
    net::{SocketAddr, TcpListener, TcpStream},
    str::FromStr,
    time::Duration,
};

use anyhow::{Result, bail};
use once_cell::sync::Lazy;
use tokio::{
    io::{AsyncBufReadExt, BufReader as TokioBufReader, BufWriter as TokioBufWriter},
    net::TcpStream as TokioTcpStream,
    spawn,
    sync::{
        Mutex,
        broadcast::{self, Receiver, Sender},
    },
    time::sleep,
};

use crate::{
    channels::Channel,
    login::send_motd,
    messages::Message,
    sender::{IrcResponse, IrcResponseCodes},
    user::{User, UserUnwrapped},
};

mod channels;
mod commands;
mod login;
mod messages;
mod sender;
mod user;

pub static CONNECTED_USERS: Lazy<Mutex<HashSet<UserUnwrapped>>> =
    Lazy::new(|| Mutex::new(HashSet::new()));
pub static JOINED_CHANNELS: Lazy<Mutex<HashSet<Channel>>> =
    Lazy::new(|| Mutex::new(HashSet::new()));
pub static SENDER: Lazy<Mutex<Option<Sender<Message>>>> = Lazy::new(|| Mutex::new(None));

#[allow(dead_code)]
#[derive(Clone)]
struct ServerInfo {
    ip: String,
    port: String,
    server_hostname: String,
    network_name: String,
    operators: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let info = ServerInfo {
        ip: "0.0.0.0".into(),
        port: "6667".into(),
        server_hostname: "irc.blah.blah".into(),
        network_name: "TeamDunno".into(),
        operators: Vec::new(),
    };
    // TODO: ^ pull these from a config file

    let listener = TcpListener::bind(SocketAddr::from_str(&format!("{}:{}", info.ip, info.port))?)?;
    let (tx, mut _rx) = broadcast::channel::<Message>(32);
    let mut sender_mut = SENDER.lock().await;
    *sender_mut = Some(tx.clone());
    drop(sender_mut);

    for stream in listener.incoming() {
        let stream = stream?;
        stream.set_nonblocking(true)?;
        let tx_thread = tx.clone();
        let info = info.clone();

        spawn(async move {
            handle_connection(stream, info, /*&mut rx_thread,*/ tx_thread)
                .await
                .unwrap()
        });
    }

    Ok(())
}

async fn handle_connection(stream: TcpStream, info: ServerInfo, tx: Sender<Message>) -> Result<()> {
    let stream_tcp = stream.try_clone()?;
    let mut message_receiver = tx.clone().subscribe();
    let mut tcp_reader = TokioBufReader::new(TokioTcpStream::from_std(stream.try_clone()?)?);
    let mut tcp_writer = TokioBufWriter::new(TokioTcpStream::from_std(stream)?);
    let mut state = User::default();

    loop {
        tokio::select! {
            result = tcp_listener(&stream_tcp, state.clone(), &info, &mut tcp_reader) => {
                match result {
                    Ok(modified_user) => {
                        state = modified_user;
                    }

                    Err(_) => {
                        break;
                    }
                }
            },
            result = message_listener(&state, &mut message_receiver, &mut tcp_writer) => {
                match result {
                    Ok(_) => {},
                    Err(_) => {
                        // break;
                    }
                }
            },
            _ = sleep(Duration::from_millis(200)) => {},
        }
    }

    stream_tcp.shutdown(std::net::Shutdown::Both)?;

    Ok(())
}

async fn tcp_listener(
    stream: &TcpStream,
    mut state: User,
    info: &ServerInfo,
    reader: &mut TokioBufReader<TokioTcpStream>,
) -> Result<User> {
    let mut buffer = String::new();

    let mut writer = TokioBufWriter::new(TokioTcpStream::from_std(stream.try_clone()?)?);

    buffer.clear();
    match reader.read_line(&mut buffer).await {
        Ok(0) => bail!("invalid response"),
        Ok(_) => {}

        Err(_) => {
            let mut conneted_users = CONNECTED_USERS.lock().await;
            let _ = conneted_users.remove(&state.clone().unwrap_all());

            bail!("client disconnected")
        }
    }

    let command = commands::IrcCommand::new(buffer.clone());
    match command
        .execute(&mut writer, &info.server_hostname, &mut state)
        .await
    {
        Ok(_) => {}
        Err(error) => {
            let error_string = format!("error processing your command: {error:#?}\n");
            let error = IrcResponseCodes::UnknownCommand;

            error
                .into_irc_response("*".into(), error_string.into())
                .send(&info.server_hostname, &mut writer, true)
                .await
                .unwrap();
        }
    }

    if !state.identified && state.is_populated() {
        send_motd(info.clone(), state.clone(), &mut writer).await?;

        state.identified = true;
        CONNECTED_USERS
            .lock()
            .await
            .insert(state.clone().unwrap_all());
    }

    Ok(state)
}

async fn message_listener(
    user_wrapped: &User,
    receiver: &mut Receiver<Message>,
    writer: &mut TokioBufWriter<TokioTcpStream>,
) -> Result<()> {
    if !user_wrapped.is_populated() {
        bail!("user has not registered yet, returning...");
    }

    let user = user_wrapped.clone().unwrap_all();
    let message: Message = receiver.recv().await.unwrap();
    let joined_channels = JOINED_CHANNELS.lock().await;

    let mut channel_name: Option<String> = None;

    println!("{message:#?}");

    match message {
        Message::PrivMessage(message) => {
            for channel in joined_channels.clone() {
                if channel.joined_users.contains(user_wrapped) && channel.name == message.receiver {
                    channel_name = Some(channel.name.clone());
                }
            }

            if user.nickname.clone().to_ascii_lowercase() == message.receiver.to_ascii_lowercase() {
                IrcResponse {
                    sender: Some(message.sender.hostmask()),
                    command: "PRIVMSG".into(),
                    arguments: Vec::new(),
                    message: message.text,
                    receiver: Some(user.username.clone()),
                }
                .send("", writer, true)
                .await?;
            } else if let Some(channel_name) = channel_name {
                if message.sender != user {
                    IrcResponse {
                        sender: Some(message.sender.hostmask()),
                        command: "PRIVMSG".into(),
                        arguments: Vec::new(),
                        message: message.text,
                        receiver: Some(channel_name),
                    }
                    .send("", writer, true)
                    .await?;
                }
            }
        }

        Message::JoinMessage(message) => {
            if message.channel.joined_users.contains(user_wrapped) || message.sender == user {
                IrcResponse {
                    sender: Some(message.sender.hostmask().clone()),
                    command: "JOIN".into(),
                    arguments: Vec::new(),
                    message: message.channel.name.clone(),
                    receiver: None,
                }
                .send("", writer, true)
                .await?;
            }
        }
    }

    Ok(())
}
