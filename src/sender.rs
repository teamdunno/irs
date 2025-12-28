use tokio::{
    io::{AsyncWriteExt, BufWriter},
    net::TcpStream,
};

use crate::error_structs::SenderError;

#[derive(Clone, Debug)]
pub struct IrcResponse {
    pub sender: Option<String>,
    pub command: String,
    pub receiver: Option<String>,
    pub arguments: Vec<String>,
    pub message: String,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum IrcResponseCodes {
    UnknownCommand = 421,
    Welcome = 001,
    YourHost = 002,
    MyInfo = 004,
    ISupport = 005,
    NoMotd = 422,
    NoTopic = 331,
    NameReply = 353,
    EndOfNames = 366,
}

impl IrcResponse {
    pub async fn send(
        &self,
        hostname: &str,
        writer: &mut BufWriter<TcpStream>,
        prepend_column: bool,
    ) -> Result<(), SenderError> {
        let sender = format!(":{}", self.sender.clone().unwrap_or(hostname.to_string()));
        let mut full_response = Vec::new();

        full_response.push(sender);
        full_response.push(self.command.clone());
        full_response.extend_from_slice(&self.arguments);
        if let Some(receiver) = self.receiver.clone() {
            full_response.push(receiver);
        }
        if prepend_column {
            full_response.push(format!(":{}\r\n", self.message.trim_end()));
        } else {
            full_response.push(format!("{}\r\n", self.message.trim_end()));
        }

        writer.write_all(full_response.join(" ").as_bytes()).await?;
        writer.flush().await?;

        println!("sending: {full_response:#?}");

        Ok(())
    }
}

impl From<IrcResponseCodes> for String {
    fn from(value: IrcResponseCodes) -> Self {
        let value = value as u16;

        value.to_string()
    }
}

impl IrcResponseCodes {
    pub fn into_irc_response(&self, receiver: String, message: String) -> IrcResponse {
        IrcResponse {
            sender: None,
            command: (*self).into(),
            arguments: Vec::new(),
            receiver: Some(receiver),
            message,
        }
    }
}
