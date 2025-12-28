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
        _server_outgoing_password: String,
        _server_incoming_passwords: Vec<String>,
        _user_passwords: Vec<String>,
    ) -> Vec<IrcAction> {
        if command.len() < 4 {
            return vec![IrcAction::DoNothing]; // XXX: return an error
        }

        // oh my god this is a mess
        user_state.username = Some({
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
        user_state.realname = Some(command[3].clone());

        vec![IrcAction::DoNothing]
    }
}
