use async_trait::async_trait;

use crate::{
    JOINED_CHANNELS,
    channels::Channel,
    commands::{IrcAction, IrcHandler},
    user::User,
};

pub struct Join;

#[async_trait]
impl IrcHandler for Join {
    async fn handle(
        &self,
        arguments: Vec<String>,
        authenticated: bool,
        user_state: &mut User,
        _server_outgoing_password: String,
        _server_incoming_passwords: Vec<String>,
        _user_passwords: Vec<String>,
    ) -> Vec<super::IrcAction> {
        let mut joined_channels = JOINED_CHANNELS.lock().await;
        let mut channels = Vec::new();

        for channel in arguments[0].clone().split(',') {
            let mut maybe_existing_channel: Option<Channel> = None;

            if !channel.starts_with("#") {
                continue;
            }

            if !authenticated {
                return vec![IrcAction::ErrorAuthenticateFirst];
            }

            for existing_channel in joined_channels.clone() {
                if existing_channel.name == channel {
                    maybe_existing_channel = Some(existing_channel);
                }
            }

            if let Some(mut new_channel) = maybe_existing_channel.clone() {
                new_channel.joined_users.insert(user_state.clone());

                joined_channels.remove(&maybe_existing_channel.clone().unwrap());
                joined_channels.insert(new_channel.clone());

                channels.push(new_channel.clone());
            } else {
                let new_channel = Channel::new_channel(channel.into(), user_state.clone());

                joined_channels.insert(new_channel.clone());

                channels.push(new_channel.clone());
            }
        }

        vec![IrcAction::JoinChannels(channels)]
    }
}
