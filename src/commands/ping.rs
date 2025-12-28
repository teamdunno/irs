use async_trait::async_trait;

use crate::{
    commands::{IrcAction, IrcHandler},
    sender::IrcResponse,
    user::User,
};

pub struct Ping;

#[async_trait]
impl IrcHandler for Ping {
    async fn handle(
        &self,
        command: Vec<String>,
        authenticated: bool,
        user_state: &mut User,
        _server_outgoing_password: String,
        _server_incoming_passwords: Vec<String>,
        _user_passwords: Vec<String>,
    ) -> Vec<IrcAction> {
        if authenticated {
            vec![IrcAction::SendText(IrcResponse {
                sender: None,
                command: "PONG".into(),
                arguments: Vec::new(),
                receiver: Some(user_state.username.clone().unwrap()),
                message: format!(":{}", command[0].clone()),
            })]
        } else {
            vec![IrcAction::DoNothing]
        }
    }
}
