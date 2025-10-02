use anyhow::Result;
use tokio::{
    io::{AsyncWriteExt, BufWriter},
    net::TcpStream,
};

#[derive(Clone)]
pub struct IrcResponse {
    pub sender: Option<String>,
    pub command: String,
    pub receiver: String,
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
}

impl IrcResponse {
    pub async fn send(
        &self,
        hostname: &str,
        writer: &mut BufWriter<TcpStream>,
        prepend_column: bool,
    ) -> Result<()> {
        let mut response = format!(
            ":{} {} {} ",
            self.sender.clone().unwrap_or(hostname.to_string()),
            self.command,
            self.receiver
        );

        if prepend_column {
            response.push_str(&format!(":{}\r\n", self.message.trim_end()));
        } else {
            response.push_str(&format!("{}\r\n", self.message.trim_end()));
        }

        writer.write_all(response.as_bytes()).await?;
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
            receiver,
            message,
        }
    }
}
