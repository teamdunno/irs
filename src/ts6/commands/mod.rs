use std::collections::HashMap;

use crate::{
    sender::IrcResponse,
    ts6::{
        ServerId, Ts6,
        commands::{capab::Capab, ping::Ping, server::Server, svinfo::Svinfo},
    },
};
use anyhow::anyhow;
use async_trait::async_trait;
use tokio::{io::BufWriter, net::TcpStream};

mod capab;
mod ping;
mod server;
mod svinfo;
mod uid;

#[derive(Clone, Debug)]
pub struct Ts6Info {
    pub sid: Option<ServerId>,
    pub hopcount: Option<u16>,
    pub description: Option<String>,
    pub name: Option<String>,
}

#[derive(Clone, Debug)]
pub enum Ts6Action {
    SetInfo(Ts6Info),
    SendText(IrcResponse),
    DoNothing,
}

#[async_trait]
pub trait Ts6Handler: Send + Sync {
    async fn handle(
        &self,
        command: Vec<String>,
        server_status: Ts6,
        my_sid: ServerId,
        hostname: &str,
    ) -> Vec<Ts6Action>;
}

#[derive(Debug)]
pub struct Ts6Command {
    command: String,
    arguments: Vec<String>,
}

impl Ts6Command {
    pub async fn new(command_with_arguments: String) -> Self {
        let mut split_command: Vec<&str> = command_with_arguments
            .split_whitespace()
            .into_iter()
            .collect();

        if split_command[0].starts_with(":") {
            split_command.remove(0);
        }

        let command = split_command[0].to_owned();
        let mut arguments = Vec::new();
        let mut buffer: Option<String> = None;

        split_command[1..]
            .iter()
            .for_each(|e| match (buffer.as_mut(), e.starts_with(":")) {
                (None, false) => arguments.push(e.to_string()),
                (None, true) => {
                    buffer = Some(e[1..].to_string());
                }
                (Some(buf), starts_with_colon) => {
                    buf.push(' ');
                    buf.push_str(if starts_with_colon { &e[1..] } else { &e });
                }
            });

        if let Some(buf) = buffer {
            arguments.push(buf.to_string());
        }

        Self {
            command: command,
            arguments: arguments,
        }
    }

    pub async fn execute(
        &self,
        ts6_status: &mut Ts6,
        hostname: &str,
        writer: &mut BufWriter<TcpStream>,
    ) -> Result<(), anyhow::Error> {
        let my_sid: String = "123".to_owned(); // TODO: generate random and allow overriding from
        // config
        let mut command_map: HashMap<String, &dyn Ts6Handler> = HashMap::new();

        command_map.insert("CAPAB".to_owned(), &Capab);
        command_map.insert("SERVER".to_owned(), &Server);
        command_map.insert("PING".to_owned(), &Ping);
        command_map.insert("SVINFO".to_owned(), &Svinfo);

        let command_to_execute = command_map
            .get(&self.command.to_uppercase())
            .map(|v| *v)
            .ok_or(anyhow!("error"))?; // TODO: error handling!!!

        let actions = command_to_execute
            .handle(
                self.arguments.clone(),
                ts6_status.clone(),
                ServerId::try_from(my_sid.clone()).unwrap(),
                hostname,
            )
            .await;

        println!("{actions:#?}");

        for action in actions {
            match action {
                Ts6Action::DoNothing => {}
                Ts6Action::SetInfo(new_info) => {
                    if let Some(sid) = new_info.sid {
                        (*ts6_status).server_id = sid;
                    };

                    if let Some(hopcount) = new_info.hopcount {
                        (*ts6_status).hopcount = hopcount;
                    };

                    if let Some(name) = new_info.name {
                        (*ts6_status).hostname = name;
                    };

                    if let Some(description) = new_info.description {
                        (*ts6_status).description = description;
                    };
                }
                Ts6Action::SendText(response) => {
                    response.send(&my_sid, writer, false).await.unwrap();
                    // TODO: error handling
                }
            }
        }

        Ok(())
    }
}
