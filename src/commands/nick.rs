use async_trait::async_trait;
use tokio::sync::broadcast::Sender;

use crate::{
    commands::{IrcAction, IrcHandler},
    messages::Message,
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
        _sender: Sender<Message>,
    ) -> IrcAction {
        user_state.nickname = Some(command[0].clone());

        IrcAction::DoNothing
    }
}
