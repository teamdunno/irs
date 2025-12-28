use async_trait::async_trait;

use crate::{
    commands::{IrcAction, IrcHandler},
    user::User,
};

pub struct Nick;

#[async_trait]
impl IrcHandler for Nick {
    async fn handle(
        &self,
        command: Vec<String>,
        _authenticated: bool,
        user_state: &mut User,
        server_outgoing_password: String,
        server_incoming_passwords: Vec<String>,
        user_passwords: Vec<String>,
    ) -> Vec<IrcAction> {
        user_state.nickname = Some(command[0].clone());

        vec![IrcAction::DoNothing]
    }
}
