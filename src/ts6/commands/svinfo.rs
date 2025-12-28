use std::time::SystemTime;

use crate::{
    sender::IrcResponse,
    ts6::{
        ServerId, Ts6,
        commands::{CommandSender, Ts6Action, Ts6Handler},
    },
};
use async_trait::async_trait;

pub struct Svinfo;

const TS_CURRENT: u8 = 6;
const TS_MINIMUM: u8 = 6;

#[async_trait]
impl Ts6Handler for Svinfo {
    async fn handle(
        &self,
        command: Vec<String>,
        _server_status: Ts6,
        _my_sid: ServerId,
        sender: Option<CommandSender>,
        _hostname: &str,
    ) -> Vec<Ts6Action> {
        let ts_current = command[0].parse::<u8>().unwrap();
        let ts_minimum = command[1].parse::<u8>().unwrap();
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // XXX: we need to properly disconnect with a QUIT message but we currently don't handle
        // that.. we probably need a Ts6Action for that. same goes for regular irc commands
        assert_eq!(ts_current, TS_CURRENT);
        assert_eq!(ts_minimum, TS_MINIMUM);

        vec![Ts6Action::SendText(IrcResponse {
            sender: None,
            command: "SVINFO".to_owned(),
            receiver: None,
            arguments: vec!["6".to_owned(), "6".to_owned(), "0".to_owned()],
            message: format!(":{}", current_time),
        })]
    }
}
