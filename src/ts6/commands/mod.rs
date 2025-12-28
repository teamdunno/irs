use std::collections::HashMap;

use crate::{
    SENDER,
    commands::IrcMessage,
    messages::Message,
    sender::IrcResponse,
    ts6::{
        ServerId, Ts6,
        commands::{
            capab::Capab, ping::Ping, privmsg::Privmsg, server::Server, svinfo::Svinfo, uid::Uid,
        },
        structs::UserId,
    },
};
use anyhow::anyhow;
use async_trait::async_trait;
use tokio::{io::BufWriter, net::TcpStream};

mod capab;
mod ping;
mod privmsg;
mod server;
mod svinfo;
mod uid;

#[derive(Clone, Debug)]
pub struct Ts6Info {
    pub sid: Option<ServerId>,
    pub hopcount: Option<u16>,
    pub description: Option<String>,
    pub name: Option<String>,

    pub identified: Option<bool>,
}

#[derive(Clone, Debug)]
pub enum Ts6Action {
    SetInfo(Ts6Info),
    SendText(IrcResponse),
    SendMessage(Message),
    DoNothing,
}

#[async_trait]
pub trait Ts6Handler: Send + Sync {
    async fn handle(
        &self,
        command: Vec<String>,
        server_status: Ts6,
        my_sid: ServerId,
        sender: Option<CommandSender>,
        hostname: &str,
    ) -> Vec<Ts6Action>;
}

#[derive(Debug)]
pub struct Ts6Command {
    command: String,
    arguments: Vec<String>,
    sender: Option<CommandSender>,
}

#[derive(Clone, Debug)]
pub enum CommandSender {
    User(UserId),
    Server(ServerId),
}

impl Ts6Command {
    pub async fn new(command_with_arguments: String) -> Self {
        let mut command_sender = None;
        let mut split_command: Vec<&str> = command_with_arguments
            .split_whitespace()
            .into_iter()
            .collect();

        if split_command[0].starts_with(":") {
            let sender = split_command.remove(0).to_string().replace(":", "");

            dbg!(&sender);

            match sender.len() {
                3 => {
                    if let Ok(sid) = ServerId::try_from(sender) {
                        command_sender = Some(CommandSender::Server(sid));
                    }
                }

                9 => {
                    if let Ok(uid) = UserId::try_from(sender) {
                        command_sender = Some(CommandSender::User(uid));
                    }
                }

                _ => {}
            }
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
            sender: command_sender,
        }
    }

    pub async fn execute(
        &self,
        ts6_status: &mut Ts6,
        hostname: &str,
        my_sid: &ServerId,
        writer: &mut BufWriter<TcpStream>,
    ) -> Result<(), anyhow::Error> {
        let mut command_map: HashMap<String, &dyn Ts6Handler> = HashMap::new();
        let message_sender = SENDER.lock().await.clone().unwrap();

        command_map.insert("CAPAB".to_owned(), &Capab);
        command_map.insert("SERVER".to_owned(), &Server);
        command_map.insert("PING".to_owned(), &Ping);
        command_map.insert("SVINFO".to_owned(), &Svinfo);
        command_map.insert("UID".to_owned(), &Uid);
        command_map.insert("PRIVMSG".to_owned(), &Privmsg);

        let command_to_execute = command_map
            .get(&self.command.to_uppercase())
            .map(|v| *v)
            .ok_or(anyhow!("error"))?; // TODO: error handling!!!

        let actions = command_to_execute
            .handle(
                self.arguments.clone(),
                ts6_status.clone(),
                ServerId::try_from(my_sid.clone().to_owned()).unwrap(),
                self.sender.clone(),
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

                    if let Some(identified) = new_info.identified {
                        (*ts6_status).identified = identified;
                    }
                }
                Ts6Action::SendText(response) => {
                    response
                        .send(&my_sid.to_string(), writer, false)
                        .await
                        .unwrap();
                    // TODO: error handling
                }
                Ts6Action::SendMessage(message) => {
                    message_sender.send(message.clone()).unwrap();
                }
            }
        }

        Ok(())
    }
}
