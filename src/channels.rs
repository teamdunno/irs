use std::collections::BTreeSet;

use anyhow::Result;
use tokio::{io::BufWriter, net::TcpStream};

use crate::{sender::IrcResponseCodes, user::User};

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Channel {
    pub name: String,
    pub joined_users: BTreeSet<User>,
}

impl Channel {
    pub fn add_user(&mut self, user: User) {
        self.joined_users.insert(user);
    }

    pub fn new_channel(name: String, user: User) -> Self {
        Channel {
            name,
            joined_users: BTreeSet::from([user]),
        }
    }

    pub async fn names_list_send(
        &self,
        user: User,
        writer: &mut BufWriter<TcpStream>,
        hostname: &str,
    ) -> Result<()> {
        let mut members = Vec::new();

        for member in self.clone().joined_users {
            members.push(member.nickname.unwrap());
        }

        IrcResponseCodes::NameReply
            .into_irc_response(
                user.nickname.clone().unwrap(),
                format!("= {} :{}", self.name.clone(), members.join(" ")),
            )
            .send(hostname, writer, false)
            .await?;
        IrcResponseCodes::EndOfNames
            .into_irc_response(
                user.nickname.clone().unwrap(),
                format!("{} :End of /NAMES list", self.name.clone()),
            )
            .send(hostname, writer, false)
            .await?;

        Ok(())
    }

    pub async fn send_topic(
        &self,
        user: User,
        writer: &mut BufWriter<TcpStream>,
        hostname: &str,
    ) -> Result<()> {
        IrcResponseCodes::NoTopic
            .into_irc_response(
                user.nickname.clone().unwrap(),
                format!("{} :No topic is set", self.name.clone()),
            )
            .send(hostname, writer, false)
            .await?;

        Ok(())
    }
}
