use async_trait::async_trait;
use tokio::sync::broadcast::Sender;

use crate::{
    commands::{IrcAction, IrcHandler},
    messages::Message,
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
        _sender: Sender<Message>,
    ) -> IrcAction {
        if command.len() < 4 {
            return IrcAction::DoNothing; // XXX: return an error
        }
        user_state.username = Some(command[0].clone());
        user_state.realname = Some(command[3].clone());

        IrcAction::DoNothing
    }
}
