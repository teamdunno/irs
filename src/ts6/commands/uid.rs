use std::{
    net::{IpAddr, Ipv4Addr},
    str::FromStr,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crate::{
    FOREIGN_CONNECTED_USERS,
    ts6::{
        ServerId, Ts6,
        commands::{CommandSender, Ts6Action, Ts6Handler},
        structs::UserId,
    },
    user::UserUnwrapped,
    usermodes::Usermodes,
};
use async_trait::async_trait;

pub struct Uid;

#[async_trait]
impl Ts6Handler for Uid {
    async fn handle(
        &self,
        command: Vec<String>,
        server_status: Ts6,
        my_sid: ServerId,
        sender: Option<CommandSender>,
        hostname: &str,
    ) -> Vec<Ts6Action> {
        let username = command[0].clone();
        let hops = command[1].clone().parse::<u16>().unwrap();
        let timestamp = UNIX_EPOCH + Duration::new(command[2].parse::<u64>().unwrap(), 0);
        let usermodes = Usermodes::default(); // XXX
        let ip = IpAddr::from_str(&command[7]).unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
        let user_id = UserId::try_from(command[8].clone()).unwrap();
        // TODO: error handling

        let user = UserUnwrapped {
            username: username.clone(),
            nickname: username.clone(),
            realname: username.clone(),
            hopcount: hops,
            identified: true,
            user_id,
            usermodes,
            timestamp,
            ip,
        };

        dbg!(&user);

        let mut foreign_users = FOREIGN_CONNECTED_USERS.lock().await;
        foreign_users.insert(user.clone());

        vec![]
    }
}
