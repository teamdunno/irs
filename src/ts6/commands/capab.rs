use crate::ts6::{
    ServerId, Ts6,
    commands::{CommandSender, Ts6Action, Ts6Handler},
};
use async_trait::async_trait;

pub struct Capab;

// TODO: handle capabilities

#[async_trait]
impl Ts6Handler for Capab {
    async fn handle(
        &self,
        command: Vec<String>,
        _server_status: Ts6,
        _my_sid: ServerId,
        sender: Option<CommandSender>,
        _hostname: &str,
    ) -> Vec<Ts6Action> {
        let args = {
            let args_without_command = command[1..].to_vec();

            if args_without_command.len() == 1 {
                args_without_command[0]
                    .split_whitespace()
                    .map(|x| x.to_owned())
                    .collect::<Vec<String>>()
            } else {
                args_without_command
            }
        };

        println!("{args:#?}");

        vec![Ts6Action::DoNothing]
    }
}
