// TODO: better error handling

use std::{net::TcpStream, time::Duration};
use tokio::{
    io::{AsyncBufReadExt, BufReader as TokioBufReader, BufWriter as TokioBufWriter},
    net::TcpStream as TokioTcpStream,
    sync::broadcast::Receiver,
    time::sleep,
};

use crate::{
    config::ServerInfo,
    messages::Message,
    ts6::{commands::Ts6Command, structs::ServerId},
    user::User,
};

#[derive(Clone, Debug, Default)]
pub struct Ts6 {
    pub server_id: ServerId,
    pub hopcount: u16,
    pub description: String,
    pub hostname: String,
}

mod commands;
pub mod structs;

impl Ts6 {
    pub async fn handle_command(
        &mut self,
        my_server_id: &ServerId,
        args: String,
        hostname: &str,
        writer: &mut TokioBufWriter<TokioTcpStream>,
    ) {
        println!("server command: {}", self.server_id);
        let args = Ts6Command::new(args).await;
        println!("args: {args:#?}");

        // XXX
        let result = args.execute(self, hostname, writer).await;
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

        let mut args = buffer
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        // This works on anope but breaks ircd-hybrid
        /*if args[0].starts_with(":") {
            // TODO: error handling
            (*server_id) = args.remove(0).replace(":", "").try_into().unwrap();
        }*/

        self_clone
            .handle_command(
                my_server_id,
                args.join(" "),
                &info.server_hostname,
                &mut writer,
            )
            .await;

        Ok(self_clone)
    }

    pub async fn message_listener(
        &self,
        user_wrapped: &User,
        receiver: &mut Receiver<Message>,
        writer: &mut TokioBufWriter<TokioTcpStream>,
        hostname: &str,
    ) -> Result<(), anyhow::Error> {
        Ok(())
    }
}
