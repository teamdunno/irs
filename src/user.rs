#![allow(dead_code)]

#[derive(Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct User {
    pub nickname: Option<String>,
    pub username: Option<String>,
    pub realname: Option<String>,
    pub identified: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UserUnwrapped {
    pub nickname: String,
    pub username: String,
    pub realname: String,
    pub identified: bool,
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
        }
    }

    pub fn default() -> Self {
        Self {
            nickname: None,
            username: None,
            realname: None,
            identified: false,
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
