use crate::{channels::Channel, user::UserUnwrapped};

#[derive(Debug, Clone)]
pub enum Message {
    PrivMessage(PrivMessage),
    JoinMessage(JoinMessage),
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JoinMessage {
    pub sender: UserUnwrapped,
    pub channel: Channel,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PrivMessage {
    pub sender: UserUnwrapped,
    pub receiver: String,
    pub text: String,
}
