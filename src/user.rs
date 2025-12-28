#![allow(dead_code)]

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
}

impl User {
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
