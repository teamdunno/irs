use async_trait::async_trait;
use tokio::sync::broadcast::Sender;

use crate::{
    commands::{IrcAction, IrcHandler},
    messages::Message,
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
        _sender: Sender<Message>,
    ) -> IrcAction {
        if authenticated {
            IrcAction::SendText(IrcResponse {
                sender: None,
                command: "PONG".into(),
                arguments: Vec::new(),
                receiver: Some(user_state.username.clone().unwrap()),
                message: format!(":{}", command[0].clone()),
            })
        } else {
            IrcAction::DoNothing
        }
    }
}
