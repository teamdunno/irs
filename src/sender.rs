use anyhow::Result;
use tokio::{
    io::{AsyncWriteExt, BufWriter},
    net::TcpStream,
};

#[derive(Clone)]
pub struct IrcResponse {
    pub sender: Option<String>,
    pub command: String,
    pub receiver: Option<String>,
    pub arguments: Vec<String>,
    pub message: String,
}

#[derive(Clone, Copy)]
pub enum IrcResponseCodes {
    UnknownCommand,
    Welcome,
    YourHost,
    MyInfo,
    ISupport,
    NoMotd,
    NoTopic,
    NameReply,
    EndOfNames,
}

impl IrcResponse {
    pub async fn send(
        &self,
        hostname: &str,
        writer: &mut BufWriter<TcpStream>,
        prepend_column: bool,
    ) -> Result<()> {
        let sender = format!(":{}", self.sender.clone().unwrap_or(hostname.to_string()));
        let mut full_response = Vec::new();

        full_response.push(sender);
        full_response.extend_from_slice(&self.arguments);
        full_response.push(self.command.clone());
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

        Ok(())
    }
}

impl From<IrcResponseCodes> for &str {
    fn from(value: IrcResponseCodes) -> Self {
        match value {
            IrcResponseCodes::UnknownCommand => "421",
            IrcResponseCodes::Welcome => "001",
            IrcResponseCodes::YourHost => "002",
            IrcResponseCodes::MyInfo => "004",
            IrcResponseCodes::ISupport => "005",
            IrcResponseCodes::NoMotd => "422",
            IrcResponseCodes::NoTopic => "331",
            IrcResponseCodes::NameReply => "353",
            IrcResponseCodes::EndOfNames => "366",
        }
    }
}

impl From<IrcResponseCodes> for String {
    fn from(value: IrcResponseCodes) -> Self {
        Into::<&str>::into(value).to_string()
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
