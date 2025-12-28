// TODO: better error handling

use std::{
    net::TcpStream,
    time::{Duration, UNIX_EPOCH},
};
use tokio::{
    io::{AsyncBufReadExt, BufReader as TokioBufReader, BufWriter as TokioBufWriter},
    net::TcpStream as TokioTcpStream,
    sync::broadcast::Receiver,
    time::sleep,
};

use crate::{
    config::ServerInfo,
    messages::Message,
    sender::IrcResponse,
    ts6::{commands::Ts6Command, structs::ServerId},
};

#[derive(Clone, Debug, Default)]
pub struct Ts6 {
    pub server_id: ServerId,
    pub hopcount: u16,
    pub description: String,
    pub hostname: String,

    identified: bool,
}

mod commands;
pub mod structs;

impl Ts6 {
    pub async fn handle_command(
        &mut self,
        _my_server_id: &ServerId,
        args: String,
        hostname: &str,
        my_sid: &ServerId,
        writer: &mut TokioBufWriter<TokioTcpStream>,
    ) {
        println!("server command: {}", self.server_id);
        let args = Ts6Command::new(args).await;
        println!("args: {args:#?}");

        // XXX
        let result = args.execute(self, hostname, my_sid, writer).await;
        if result.is_err() {
            println!("{result:#?}");
        }
    }

    pub async fn tcp_listener(
        &self,
        stream: &TcpStream,
        info: &ServerInfo,
        reader: &mut TokioBufReader<TokioTcpStream>,
        my_server_id: &ServerId,
    ) -> Result<Ts6, anyhow::Error> {
        let mut buffer = String::new();
        let mut self_clone = self.clone();

        let mut writer = TokioBufWriter::new(TokioTcpStream::from_std(stream.try_clone()?)?);

        match reader.read_line(&mut buffer).await {
            Ok(0) => anyhow::bail!(""),
            Ok(_) => {}

            Err(_) => {
                anyhow::bail!("");
            }
        }

        println!("ts6: {buffer}");

        let args = buffer
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        self_clone
            .handle_command(
                my_server_id,
                args.join(" "),
                &info.server_hostname,
                my_server_id,
                &mut writer,
            )
            .await;

        Ok(self_clone)
    }

    pub async fn message_listener(
        &self,
        receiver: &mut Receiver<Message>,
        writer: &mut TokioBufWriter<TokioTcpStream>,
        my_sid: &ServerId,
        hostname: &str,
    ) -> Result<(), anyhow::Error> {
        if !self.identified {
            sleep(Duration::from_millis(250)).await; // avoid immediate returns b'cuz they result in high
            // cpu usage
            return Ok(()); // TODO: error handling
        }

        let message: Message = receiver.recv().await.unwrap();

        match message {
            Message::NetJoinMessage(net_join_message) => {
                let user = net_join_message.user.clone();

                // TODO: refactor this entire thing. we need hostmask and ip and such fully working
                IrcResponse {
                    sender: Some(my_sid.clone().to_string()),
                    command: "UID".to_string(),
                    receiver: None,
                    arguments: vec![
                        user.nickname.clone(),
                        (user.hopcount + 1).to_string(),
                        user.timestamp
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs()
                            .to_string(),
                        user.usermodes.into(),
                        format!("~{}", user.username.clone()),
                        user.ip.to_string(),
                        user.ip.to_string(),
                        user.ip.to_string(),
                        user.user_id.to_string().clone(),
                        "*".to_owned(),
                        format!(":{}", user.username.clone()),
                    ],
                    message: String::new(),
                }
                .send(hostname, writer, false)
                .await
                .unwrap();
            }

            _ => {}
        }

        Ok(())
    }
}
