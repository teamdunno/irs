use async_trait::async_trait;

use crate::{
    commands::{IrcAction, IrcHandler},
    user::User,
};

pub struct Who;

#[async_trait]
impl IrcHandler for Who {
    async fn handle(
        &self,
        _arguments: Vec<String>,
        _authenticated: bool,
        _user_state: &mut User,
        _server_outgoing_password: String,
        _server_incoming_passwords: Vec<String>,
        _user_passwords: Vec<String>,
    ) -> Vec<super::IrcAction> {
        vec![IrcAction::DoNothing] // TODO
    }
}
