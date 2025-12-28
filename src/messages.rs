use crate::{
    channels::Channel,
    ts6::structs::{ServerId, UserId},
    user::UserUnwrapped,
};

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
    pub receiver: Receiver,
    pub text: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Receiver {
    Username(String),
    UserId(UserId),
    ChannelName(String),
}
