use std::{
    clone,
    collections::HashSet,
    net::{SocketAddr, TcpListener, TcpStream},
    str::FromStr,
    time::{Duration, SystemTime},
};

use anyhow::Error as AnyhowError;
use clap::Parser;
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
use tracing::instrument;

use crate::{
    channels::Channel,
    config::ServerInfo,
    error_structs::{HandlerError, ListenerError},
    login::send_motd,
    messages::Receiver as MsgReceiver,
    messages::{Message, NetJoinMessage},
    sender::{IrcResponse, IrcResponseCodes},
    ts6::{
        Ts6,
        structs::{ServerId, UserId},
    },
    user::{User, UserUnwrapped},
};

mod channels;
mod commands;
mod config;
mod error_structs;
mod login;
mod messages;
mod sender;
mod ts6;
mod user;
mod userid_gen;
mod usermodes;

pub static CONNECTED_USERS: Lazy<Mutex<HashSet<UserUnwrapped>>> =
    Lazy::new(|| Mutex::new(HashSet::new()));
pub static FOREIGN_CONNECTED_USERS: Lazy<Mutex<HashSet<UserUnwrapped>>> =
    Lazy::new(|| Mutex::new(HashSet::new()));
pub static JOINED_CHANNELS: Lazy<Mutex<HashSet<Channel>>> =
    Lazy::new(|| Mutex::new(HashSet::new()));
pub static SENDER: Lazy<Mutex<Option<Sender<Message>>>> = Lazy::new(|| Mutex::new(None));

/// An IRCd written in Rust
#[derive(Parser, Debug)]
struct Args {
    /// Path to the config file
    #[arg(short, long)]
    pub config_path: Option<String>,
}

enum TcpListenerResult {
    UpdatedUser(User),
    ServerConnectionInit,
}

#[tokio::main]
async fn main() -> Result<(), AnyhowError> {
    #[cfg(feature = "tokio-console")]
    console_subscriber::init();

    let args = Args::parse();
    let info = ServerInfo::load(args.config_path).unwrap();
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

        spawn(handle_connection(
            stream, info, /*&mut rx_thread,*/ tx_thread,
        ));
    }

    Ok(())
}

#[instrument]
async fn handle_connection(
    stream: TcpStream,
    info: ServerInfo,
    tx: Sender<Message>,
) -> Result<(), HandlerError> {
    let stream_tcp = stream.try_clone()?;
    let mut message_receiver = tx.clone().subscribe();
    let mut tcp_reader = TokioBufReader::new(TokioTcpStream::from_std(stream.try_clone()?)?);
    let mut tcp_writer = TokioBufWriter::new(TokioTcpStream::from_std(stream)?);

    'connection_handler: {
        let mut state = User::default();

        let hostname = info.server_hostname.clone();

        // TODO: generate randomally and allow overriding from config
        let my_server_id = ServerId::try_from("000".to_owned()).unwrap();

        loop {
            tokio::select! {
                result = tcp_listener(&stream_tcp, state.clone(), &info, &mut tcp_reader, my_server_id.clone()) => {
                    match result {
                        Ok(tcp_listener_result) => {
                            match tcp_listener_result {
                                TcpListenerResult::UpdatedUser(user) => {
                                    state = user;
                                }

                                TcpListenerResult::ServerConnectionInit => {
                                    break;
                                }
                            }
                        }

                        Err(_) => {
                            break 'connection_handler;
                        }
                    }
                },
                result = message_listener(&state, &mut message_receiver, &mut tcp_writer, &hostname) => {
                    match result {
                        Ok(_) => {},
                        Err(err) => {
                            match err {
                                ListenerError::ConnectionError => {
                                    break 'connection_handler;
                                }

                                _ => {}
                            };
                        }
                    }
                },
            }
        }

        println!("upgrade to server connection");

        let mut ts6_server_status = Ts6::default();

        loop {
            tokio::select! {
                result = ts6_server_status.tcp_listener(&stream_tcp, &info, &mut tcp_reader, &my_server_id) => {
                    match result {
                        Ok(new_status) => {
                            println!("{new_status:#?}");
                            ts6_server_status = new_status;
                        },
                        Err(_) => {
                            break;
                        }
                    }
                },
                result = ts6_server_status.message_listener(&mut message_receiver, &mut tcp_writer, &my_server_id, &hostname) => {
                    match result {
                        Ok(_) => {},
                        Err(_) => {
                            break;
                        }
                    }
                },
            }
        }
    }

    stream_tcp.shutdown(std::net::Shutdown::Both)?;

    Ok(())
}

async fn tcp_listener(
    stream: &TcpStream,
    mut user_state: User,
    info: &ServerInfo,
    reader: &mut TokioBufReader<TokioTcpStream>,
    our_sid: ServerId,
) -> Result<TcpListenerResult, ListenerError> {
    let mut buffer = String::new();

    let mut writer = TokioBufWriter::new(TokioTcpStream::from_std(stream.try_clone()?)?);

    match reader.read_line(&mut buffer).await {
        Ok(0) => return Err(ListenerError::ConnectionError),
        Ok(_) => {}

        Err(_) => {
            let mut conneted_users = CONNECTED_USERS.lock().await;
            let _ = conneted_users.remove(&user_state.clone().unwrap_all());

            return Err(ListenerError::ConnectionError);
        }
    }

    let command = commands::IrcCommand::new(buffer.clone()).await;
    match command
        .execute(&mut writer, &info.server_hostname, &mut user_state, info)
        .await
    {
        Ok(return_actions) => {
            for return_action in return_actions {
                match return_action {
                    commands::ReturnAction::ServerConn => {
                        return Ok(TcpListenerResult::ServerConnectionInit);
                    }

                    _ => {}
                }
            }
        }
        Err(error) => match error {
            error_structs::CommandExecError::NonexistantCommand => {
                let error_string = format!("error processing your command: {error:#?}\n");
                let error = IrcResponseCodes::UnknownCommand;

                error
                    .into_irc_response("*".into(), error_string.into())
                    .send(&info.server_hostname, &mut writer, true)
                    .await
                    .unwrap();
            }
        },
    }

    if !user_state.identified && user_state.is_populated_without_uid() {
        let id = userid_gen::increase_user_id()
            .await
            .unwrap()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("");
        let user_id = format!("{our_sid}{id}");

        user_state.identified = true;
        user_state.user_id = Some(UserId::try_from(user_id).unwrap()); // XXX: error handling
        user_state.timestamp = Some(SystemTime::now());

        send_motd(info.clone(), user_state.clone(), &mut writer).await?;

        let broadcast_sender = SENDER.lock().await.clone().unwrap();

        broadcast_sender
            .send(Message::NetJoinMessage(NetJoinMessage {
                user: user_state.clone().unwrap_all(),
                server_id: our_sid.clone(),
            }))
            .unwrap();

        CONNECTED_USERS
            .lock()
            .await
            .insert(user_state.clone().unwrap_all());
    }

    Ok(TcpListenerResult::UpdatedUser(user_state))
}

async fn message_listener(
    user_wrapped: &User,
    receiver: &mut Receiver<Message>,
    writer: &mut TokioBufWriter<TokioTcpStream>,
    hostname: &str,
) -> Result<(), ListenerError> {
    if !user_wrapped.is_populated() {
        sleep(Duration::from_millis(250)).await; // avoid immediate returns b'cuz they result in high
        // cpu usage
        return Err(ListenerError::UserIsUnidentified);
    }

    let user = user_wrapped.clone().unwrap_all();
    let message: Message = receiver.recv().await.unwrap();
    let joined_channels = JOINED_CHANNELS.lock().await;

    let mut channel_name: Option<String> = None;

    println!("{message:#?}");

    match message {
        Message::PrivMessage(message) => {
            for channel in joined_channels.clone() {
                if let MsgReceiver::ChannelName(channelname) = message.clone().receiver
                    && channelname == channel.name
                    && channel.joined_users.contains(user_wrapped)
                {
                    channel_name = Some(channel.name.clone());
                }
            }

            dbg!(&message);

            if match message.clone().receiver {
                MsgReceiver::UserId(userid) => {
                    println!("{userid} ?= {}", user.user_id);
                    if userid == user.user_id { true } else { false }
                }

                MsgReceiver::Username(username) => {
                    if username.to_lowercase() == user.username.to_lowercase() {
                        true
                    } else {
                        false
                    }
                }

                _ => false,
            } {
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

        Message::ChanJoinMessage(message) => {
            if message.channel.joined_users.contains(user_wrapped) || message.sender == user {
                let channel = message.channel.clone();

                IrcResponse {
                    sender: Some(message.sender.hostmask().clone()),
                    command: "JOIN".into(),
                    arguments: Vec::new(),
                    message: message.channel.name.clone(),
                    receiver: None,
                }
                .send("", writer, true)
                .await?;

                channel
                    .send_topic(user_wrapped.clone(), writer, hostname)
                    .await
                    .unwrap();

                channel
                    .names_list_send(user_wrapped.clone(), &channel, writer, hostname)
                    .await
                    .unwrap();
            }
        }

        Message::NetJoinMessage(_) => {} // we don't care about these here :)
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::userid_gen;

    #[tokio::test]
    async fn test_user_id_generator() {
        while let Ok(userid) = userid_gen::increase_user_id().await {
            if userid == ['A', 'B', 'C', 'D', 'E', 'F'] {
                userid_gen::manually_set_user_id(['Z', 'Z', 'Z', 'Z', 'Z', 'Y'].to_vec()).await;
                break;
            }

            dbg!(userid);
        }

        while let Ok(userid) = userid_gen::increase_user_id().await {
            if userid == ['A', '1', '2', '3', '4', '5'] {
                // ff a bit
                userid_gen::manually_set_user_id(['Z', '1', '2', '3', '4', '5'].to_vec()).await;
            }

            dbg!(userid);
        }
    }
}
