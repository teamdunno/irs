use crate::{
    commands::{IrcAction, IrcHandler},
    sender::IrcResponse,
};

pub struct Cap;

impl IrcHandler for Cap {
    fn handle(&self, _arguments: Vec<String>) -> super::IrcAction {
        // TODO: parse the args, etc
        IrcAction::SendText(IrcResponse {
            command: "CAP".into(),
            receiver: "*".into(),
            arguments: Some("LS".into()),
            message: "TODO".into(),
        })
    }
}
