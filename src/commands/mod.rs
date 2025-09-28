#![allow(dead_code)]
use std::{collections::HashMap, io::BufWriter, net::TcpStream};

use anyhow::{Result, anyhow};

use crate::{commands::cap::Cap, sender::IrcResponse};

mod cap;

pub struct IrcCommand {
    command: String,
    arguments: Vec<String>,
}

pub enum IrcAction {
    MultipleActions(Vec<Self>),
    ModifyDatabase(DatabaseAction),
    SendText(IrcResponse),
}

pub enum DatabaseAction {}

pub trait IrcHandler {
    fn handle(&self, command: Vec<String>) -> IrcAction;
}

impl IrcCommand {
    pub fn new(command_with_arguments: String) -> Self {
        let split_command: Vec<&str> = command_with_arguments
            .split_whitespace()
            .into_iter()
            .collect();
        let command = split_command[0].to_owned();
        let mut arguments = Vec::new();

        split_command[1..]
            .iter()
            .for_each(|e| arguments.push(e.to_string()));

        Self {
            command: command,
            arguments: arguments,
        }
    }

    pub fn execute(&self, writer: &mut BufWriter<&TcpStream>, hostname: &str) -> Result<()> {
        let mut command_map: HashMap<String, &dyn IrcHandler> = HashMap::new();

        // Command map is defined here
        command_map.insert("CAP".to_owned(), &Cap);

        let command_to_execute = command_map
            .get(&self.command.to_uppercase())
            .map(|v| *v)
            .ok_or(anyhow!("unknown command!"))?;

        let action = command_to_execute.handle(self.arguments.clone());
        action.execute(writer, hostname);

        Ok(())
    }
}

impl IrcAction {
    pub fn execute(&self, writer: &mut BufWriter<&TcpStream>, hostname: &str) {
        match self {
            IrcAction::MultipleActions(actions) => {
                for action in actions {
                    action.execute(writer, hostname);
                }
            }

            IrcAction::SendText(msg) => {
                msg.send(hostname, writer).unwrap();
            }

            _ => {}
        }
    }
}
