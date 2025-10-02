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
    ) -> IrcAction {
        if authenticated {
            IrcAction::SendText(IrcResponse {
                sender: None,
                command: "PONG".into(),
                receiver: user_state.nickname.clone().unwrap(),
                message: command[0].clone(),
            })
        } else {
            IrcAction::DoNothing
        }
    }
}
