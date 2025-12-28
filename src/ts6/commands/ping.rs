use async_trait::async_trait;

use crate::{
    sender::IrcResponse,
    ts6::{
        ServerId, Ts6,
        commands::{CommandSender, Ts6Action, Ts6Handler},
    },
};

pub struct Ping;

#[async_trait]
impl Ts6Handler for Ping {
    async fn handle(
        &self,
        command: Vec<String>,
        _server_status: Ts6,
        my_sid: ServerId,
        sender: Option<CommandSender>,
        _hostname: &str,
    ) -> Vec<Ts6Action> {
        vec![Ts6Action::SendText(IrcResponse {
            sender: None,
            command: "PONG".into(),
            arguments: Vec::new(),
            receiver: None,
            message: format!("{my_sid} {}", command[0].clone()),
        })]
    }
}
