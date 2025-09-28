use std::{
    io::{BufWriter, Write},
    net::TcpStream,
};

use anyhow::Result;

pub struct IrcResponse {
    pub command: String,
    pub receiver: String,
    pub arguments: Option<String>,
    pub message: String,
}

#[derive(Clone, Copy)]
pub enum IrcResponseCodes {
    UnknownCommand,
}

impl IrcResponse {
    pub fn send(&self, hostname: &str, writer: &mut BufWriter<&TcpStream>) -> Result<()> {
        let mut response = format!(":{} {} {} ", hostname, self.command, self.receiver);

        if let Some(arguments) = &self.arguments {
            response.push_str(&format!("{} ", arguments));
        };

        response.push_str(&format!(":{}\n", self.message.trim_end()));

        writer.write_all(response.as_bytes())?;
        writer.flush()?;

        Ok(())
    }
}

impl From<IrcResponseCodes> for u32 {
    fn from(value: IrcResponseCodes) -> Self {
        match value {
            IrcResponseCodes::UnknownCommand => 421,
        }
    }
}

impl From<IrcResponseCodes> for String {
    fn from(value: IrcResponseCodes) -> Self {
        Into::<u32>::into(value).to_string()
    }
}

impl IrcResponseCodes {
    pub fn into_irc_response(&self, receiver: String, message: String) -> IrcResponse {
        IrcResponse {
            command: (*self).into(),
            receiver,
            arguments: None,
            message,
        }
    }
}
