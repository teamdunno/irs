use async_trait::async_trait;

use crate::{
    commands::{IrcAction, IrcHandler},
    user::User,
};

pub struct Cap;

#[async_trait]
impl IrcHandler for Cap {
    async fn handle(
        &self,
        _arguments: Vec<String>,
        _authenticated: bool,
        _user_state: &mut User,
    ) -> super::IrcAction {
        IrcAction::DoNothing
    }
}
