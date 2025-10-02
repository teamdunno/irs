#![allow(dead_code)]
use std::collections::HashMap;

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use tokio::{io::BufWriter, net::TcpStream};

use crate::{
    commands::{cap::Cap, nick::Nick, ping::Ping, privmsg::PrivMsg, user::User as UserHandler},
    sender::IrcResponse,
    user::User,
};

mod cap;
mod nick;
mod ping;
mod privmsg;
mod user;

#[derive(Debug)]
pub struct IrcCommand {
    command: String,
    arguments: Vec<String>,
}

pub struct IrcMessage {
    pub sender: String, // TODO: replace with hostmask
    pub message: String,
}

pub enum IrcAction {
    MultipleActions(Vec<Self>),
    SendText(IrcResponse),
    ErrorAuthenticateFirst,
    DoNothing,
}

pub enum DatabaseAction {}

#[async_trait]
pub trait IrcHandler: Send + Sync {
    async fn handle(
        &self,
        command: Vec<String>,
        authenticated: bool,
        user_state: &mut User,
    ) -> IrcAction;
}

pub struct SendMessage(Option<String>);

impl IrcCommand {
    pub fn new(command_with_arguments: String) -> Self {
        let split_command: Vec<&str> = command_with_arguments
            .split_whitespace()
            .into_iter()
            .collect();
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
        writer: &mut BufWriter<TcpStream>,
        hostname: &str,
        user_state: &mut User,
    ) -> Result<()> {
        let mut command_map: HashMap<String, &dyn IrcHandler> = HashMap::new();

        // Command map is defined here
        command_map.insert("CAP".to_owned(), &Cap);
        command_map.insert("NICK".to_owned(), &Nick);
        command_map.insert("USER".to_owned(), &UserHandler);
        command_map.insert("PRIVMSG".to_owned(), &PrivMsg);
        command_map.insert("PING".to_owned(), &Ping);

        println!("{self:#?}");

        let command_to_execute = command_map
            .get(&self.command.to_uppercase())
            .map(|v| *v)
            .ok_or(anyhow!("unknown command!"))?;

        let action = command_to_execute
            .handle(
                self.arguments.clone(),
                user_state.is_populated(),
                user_state,
            )
            .await;
        action.execute(writer, hostname).await;

        Ok(())
    }
}

impl IrcAction {
    pub async fn execute(&self, writer: &mut BufWriter<TcpStream>, hostname: &str) {
        match self {
            IrcAction::SendText(msg) => {
                msg.send(hostname, writer, false).await.unwrap();
            }

            _ => {}
        }
    }
}
