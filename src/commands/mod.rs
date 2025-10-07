#![allow(dead_code)]
use std::collections::HashMap;

use async_trait::async_trait;
use tokio::{io::BufWriter, net::TcpStream, sync::broadcast::Sender};

use crate::{
    SENDER,
    channels::Channel,
    commands::{
        cap::Cap, join::Join, nick::Nick, ping::Ping, privmsg::PrivMsg, user::User as UserHandler,
    },
    error_structs::CommandExecError,
    messages::Message,
    sender::IrcResponse,
    user::User,
};

mod cap;
mod join;
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
    SendText(IrcResponse),
    JoinChannels(Vec<Channel>),
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
        sender: Sender<Message>,
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
    ) -> Result<(), CommandExecError> {
        let mut command_map: HashMap<String, &dyn IrcHandler> = HashMap::new();
        let broadcast_sender = SENDER.lock().await.clone().unwrap();

        // Command map is defined here
        command_map.insert("CAP".to_owned(), &Cap);
        command_map.insert("NICK".to_owned(), &Nick);
        command_map.insert("USER".to_owned(), &UserHandler);
        command_map.insert("PRIVMSG".to_owned(), &PrivMsg);
        command_map.insert("PING".to_owned(), &Ping);
        command_map.insert("JOIN".to_owned(), &Join);

        println!("{self:#?}");

        let command_to_execute = command_map
            .get(&self.command.to_uppercase())
            .map(|v| *v)
            .ok_or(CommandExecError::NonexistantCommand)?;

        let action = command_to_execute
            .handle(
                self.arguments.clone(),
                user_state.is_populated(),
                user_state,
                broadcast_sender,
            )
            .await;
        action.execute(writer, hostname, &user_state).await;

        Ok(())
    }
}

impl IrcAction {
    pub async fn execute(
        &self,
        writer: &mut BufWriter<TcpStream>,
        hostname: &str,
        user_state: &User,
    ) {
        match self {
            IrcAction::SendText(msg) => {
                msg.send(hostname, writer, false).await.unwrap();
            }

            IrcAction::JoinChannels(channels) => {
                for channel in channels {
                    channel
                        .send_topic(user_state.clone(), writer, hostname)
                        .await
                        .unwrap();

                    channel
                        .names_list_send(user_state.clone(), writer, hostname)
                        .await
                        .unwrap();
                }
            }

            _ => {}
        }
    }
}
