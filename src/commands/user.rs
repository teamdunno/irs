use async_trait::async_trait;

use crate::{
    commands::{IrcAction, IrcHandler},
    user::User as UserState,
};

pub struct User;

#[async_trait]
impl IrcHandler for User {
    async fn handle(
        &self,
        command: Vec<String>,
        _authenticated: bool,
        user_state: &mut UserState,
    ) -> IrcAction {
        user_state.username = Some(command[0].clone());
        user_state.realname = Some(command[3].clone());

        IrcAction::DoNothing
    }
}
