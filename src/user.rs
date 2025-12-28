#![allow(dead_code)]

use std::{
    net::{IpAddr, Ipv4Addr},
    time::SystemTime,
};

use crate::{ts6::structs::UserId, usermodes::Usermodes};

#[derive(Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct User {
    pub nickname: Option<String>,
    pub username: Option<String>,
    pub realname: Option<String>,
    pub identified: bool,
    pub hopcount: Option<u16>,
    pub user_id: Option<UserId>,
    pub usermodes: Usermodes,
    pub timestamp: Option<SystemTime>,
    pub ip: Option<IpAddr>,
    // pub hostname: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UserUnwrapped {
    pub nickname: String,
    pub username: String,
    pub realname: String,
    pub identified: bool,
    pub hopcount: u16,
    pub user_id: UserId,
    pub usermodes: Usermodes,
    pub timestamp: SystemTime,
    pub ip: IpAddr,
    // pub hostname: Option<String>,
}

impl User {
    pub fn is_populated_without_uid(&self) -> bool {
        self.realname.is_some() && self.username.is_some() && self.nickname.is_some()
    }

    pub fn is_populated(&self) -> bool {
        self.realname.is_some() && self.username.is_some() && self.nickname.is_some()
    }

    pub fn unwrap_all(&self) -> UserUnwrapped {
        UserUnwrapped {
            nickname: self.nickname.clone().unwrap(),
            username: self.username.clone().unwrap(),
            realname: self.realname.clone().unwrap(),
            identified: self.identified,
            hopcount: self.hopcount.clone().unwrap(),
            user_id: self.user_id.clone().unwrap(),
            usermodes: self.usermodes.clone(),
            timestamp: self.timestamp.clone().unwrap(),
            ip: self.ip.unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
        }
    }

    pub fn default() -> Self {
        Self {
            nickname: None,
            username: None,
            realname: None,
            identified: false,
            hopcount: Some(0),
            user_id: None,
            usermodes: Usermodes::default(),
            timestamp: None,
            ip: None,
        }
    }
}

impl UserUnwrapped {
    pub fn hostmask(&self) -> String {
        format!(
            "{}!~{}@{}",
            self.nickname.clone(),
            self.username.clone(),
            "unimplement.ed"
        )
    }
}
