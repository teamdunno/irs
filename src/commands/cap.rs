use async_trait::async_trait;
use tokio::sync::broadcast::Sender;

use crate::{
    commands::{IrcAction, IrcHandler},
    messages::Message,
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
        _sender: Sender<Message>,
    ) -> super::IrcAction {
        IrcAction::DoNothing
    }
}
