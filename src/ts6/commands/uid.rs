use crate::ts6::{
    ServerId, Ts6,
    commands::{Ts6Action, Ts6Handler},
};
use async_trait::async_trait;

pub struct Uid;

#[async_trait]
impl Ts6Handler for Uid {
    async fn handle(
        &self,
        command: Vec<String>,
        _server_status: Ts6,
        my_sid: ServerId,
        hostname: &str,
    ) -> Vec<Ts6Action> {
        vec![]
    }
}
