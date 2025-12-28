use async_trait::async_trait;

use crate::{
    FOREIGN_CONNECTED_USERS,
    messages::{PrivMessage, Receiver},
    ts6::{
        Ts6,
        commands::{CommandSender, Ts6Action, Ts6Handler},
        structs::{ServerId, UserId},
    },
    user::UserUnwrapped,
};

pub struct Privmsg;

#[async_trait]
impl Ts6Handler for Privmsg {
    async fn handle(
        &self,
        command: Vec<String>,
        server_status: Ts6,
        my_sid: ServerId,
        sender: Option<CommandSender>,
        hostname: &str,
    ) -> Vec<Ts6Action> {
        'priv_msg: {
            let mut sending_user: Option<UserUnwrapped> = None;

            dbg!(&sender);

            if let Ok(user_id) = UserId::try_from(command[0].clone()) {
                if let Some(CommandSender::User(command_sender)) = sender {
                    let foreign_users = FOREIGN_CONNECTED_USERS.lock().await;

                    for user in foreign_users.iter() {
                        if user.user_id == command_sender {
                            sending_user = Some(user.clone())
                        }
                    }
                } else {
                    dbg!("sender");
                    break 'priv_msg vec![];
                }

                vec![Ts6Action::SendMessage(
                    crate::messages::Message::PrivMessage(PrivMessage {
                        sender: sending_user.unwrap(),
                        receiver: Receiver::UserId(user_id),
                        text: command[1].clone(),
                    }),
                )]
            } else {
                dbg!("userid");
                vec![]
            }
        }
    }
}
