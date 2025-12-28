use async_trait::async_trait;

use crate::{
    commands::{IrcAction, IrcHandler},
    user::User,
};

pub struct Pass;

#[async_trait]
impl IrcHandler for Pass {
    async fn handle(
        &self,
        command: Vec<String>,
        _authenticated: bool,
        _user_state: &mut User,
        server_outgoing_password: String,
        server_incoming_passwords: Vec<String>,
        _user_passwords: Vec<String>,
    ) -> Vec<IrcAction> {
        if server_incoming_passwords.contains(&command[0]) {
            vec![
                IrcAction::SendText(crate::sender::IrcResponse {
                    sender: None,
                    command: "PASS".to_owned(),
                    receiver: None,
                    arguments: Vec::new(),
                    message: server_outgoing_password.clone(),
                }),
                IrcAction::UpgradeToServerConn,
            ]
        } else {
            vec![IrcAction::DoNothing]
        }
    }
}
