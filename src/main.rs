use std::{
    io::{BufRead, BufReader, BufWriter},
    net::{SocketAddr, TcpListener, TcpStream},
    str::FromStr,
};

use anyhow::Result;
use tokio::spawn;

use crate::sender::IrcResponseCodes;

mod commands;
mod sender;

#[tokio::main]
async fn main() -> Result<()> {
    let ip = "0.0.0.0";
    let port = "6667";
    let server_hostname = "irc.blah.blah";
    // TODO: ^ pull these from a config file

    let listener = TcpListener::bind(SocketAddr::from_str(&format!("{}:{}", ip, port))?)?;

    for stream in listener.incoming() {
        let stream = stream?;

        spawn(async move { handle_connection(stream, server_hostname).await.unwrap() });
    }

    Ok(())
}

async fn handle_connection(stream: TcpStream, hostname: &str) -> Result<()> {
    let reader_stream = stream.try_clone()?;
    let mut reader = BufReader::new(&reader_stream);
    let mut writer = BufWriter::new(&stream);
    let mut buffer = String::new();

    loop {
        buffer.clear();
        if reader.read_line(&mut buffer).unwrap() == 0 {
            break;
        }

        let command = commands::IrcCommand::new(buffer.clone());
        match command.execute(&mut writer, hostname) {
            Ok(_) => {}
            Err(error) => {
                let error_string = format!("error processing your command: {error:#?}\n");
                let error = IrcResponseCodes::UnknownCommand;

                error
                    .into_irc_response("*".into(), error_string.into())
                    .send(hostname, &mut writer)
                    .unwrap();
            }
        }
    }

    Ok(())
}
