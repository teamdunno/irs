use crate::{channels::Channel, ts6::structs::ServerId, user::UserUnwrapped};

#[derive(Debug, Clone)]
pub enum Message {
    PrivMessage(PrivMessage),
    ChanJoinMessage(ChanJoinMessage),
    NetJoinMessage(NetJoinMessage),
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChanJoinMessage {
    pub sender: UserUnwrapped,
    pub channel: Channel,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NetJoinMessage {
    pub user: UserUnwrapped,
    pub server_id: ServerId,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PrivMessage {
    pub sender: UserUnwrapped,
    pub receiver: String,
    pub text: String,
}
