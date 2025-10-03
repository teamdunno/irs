use async_trait::async_trait;
use tokio::sync::broadcast::Sender;

use crate::{
    CONNECTED_USERS,
    commands::{IrcAction, IrcHandler},
    messages::{Message, PrivMessage},
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
        sender: Sender<Message>,
    ) -> IrcAction {
        if !authenticated {
            return IrcAction::ErrorAuthenticateFirst;
        }
        let connected_users = CONNECTED_USERS.lock().await;

        println!("{connected_users:#?}");
        drop(connected_users);

        let message = PrivMessage {
            sender: user_state.clone().unwrap_all(),
            receiver: command[0].clone(),
            text: command[1].clone(),
        };
        println!("SENDING: {message:#?}");
        sender.send(Message::PrivMessage(message)).unwrap();

        IrcAction::DoNothing
    }
}
