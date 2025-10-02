use async_trait::async_trait;

use crate::{
    CONNECTED_USERS, SENDER,
    commands::{IrcAction, IrcHandler},
    messages::Message,
    user::User,
};

pub struct PrivMsg;

#[async_trait]
impl IrcHandler for PrivMsg {
    async fn handle(
        &self,
        command: Vec<String>,
        authenticated: bool,
        user_state: &mut User,
    ) -> IrcAction {
        if !authenticated {
            return IrcAction::ErrorAuthenticateFirst;
        }
        let connected_users = CONNECTED_USERS.lock().await;
        let sender = SENDER.lock().await.clone().unwrap();

        println!("{connected_users:#?}");
        drop(connected_users);

        let message = Message {
            sender: user_state.clone().unwrap_all(),
            receiver: command[0].clone(),
            text: command[1].clone(),
        };
        println!("SENDING: {message:#?}");
        sender.send(message).unwrap();

        IrcAction::DoNothing
    }
}
