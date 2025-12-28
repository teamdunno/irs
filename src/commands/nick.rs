use async_trait::async_trait;

use crate::{
    commands::{IrcAction, IrcHandler},
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
        _server_outgoing_password: String,
        _server_incoming_passwords: Vec<String>,
        _user_passwords: Vec<String>,
    ) -> Vec<IrcAction> {
        user_state.nickname = Some({
            if command[0].len() > 9 {
                String::from_utf8(
                    command[0]
                        .clone()
                        .chars()
                        .map(|x| x.clone() as u8)
                        .collect::<Vec<u8>>()[0..8]
                        .to_vec(),
                )
                .unwrap()
            } else {
                command[0].clone()
            }
        });

        vec![IrcAction::DoNothing]
    }
}
