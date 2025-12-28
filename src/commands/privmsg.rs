use async_trait::async_trait;

use crate::{
    commands::{IrcAction, IrcHandler},
    messages::{Message, PrivMessage, Receiver},
    user::User,
};

pub struct PrivMsg;

#[async_trait]
impl IrcHandler for PrivMsg {
    async fn handle(
        &self,
        command: Vec<String>,
        authenticated: bool,
        user_state: &mut User,
        _server_outgoing_password: String,
        _server_incoming_passwords: Vec<String>,
        _user_passwords: Vec<String>,
    ) -> Vec<IrcAction> {
        if !authenticated {
            return vec![IrcAction::ErrorAuthenticateFirst];
        }

        let receiver = if command[0].clone().starts_with("#") {
            Receiver::ChannelName(command[0].clone())
        } else {
            Receiver::Username(command[0].clone())
        };

        let message = PrivMessage {
            sender: user_state.clone().unwrap_all(),
            receiver,
            text: command[1].clone(),
        };

        vec![IrcAction::SendMessage(Message::PrivMessage(message))]
    }
}
