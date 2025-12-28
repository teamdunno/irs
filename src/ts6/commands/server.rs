use crate::ts6::{
    ServerId, Ts6,
    commands::{CommandSender, Ts6Action, Ts6Handler},
};
use async_trait::async_trait;

pub struct Server;

// TODO: handle *ALL* params

#[async_trait]
impl Ts6Handler for Server {
    async fn handle(
        &self,
        command: Vec<String>,
        _server_status: Ts6,
        my_sid: ServerId,
        sender: Option<CommandSender>,
        hostname: &str,
    ) -> Vec<Ts6Action> {
        let name = Some(command[0].clone());
        let hopcount = Some(command[1].parse::<u16>().unwrap());
        let sid = {
            if let Some(sid) = ServerId::try_from(command[2].clone()).ok() {
                Some(sid)
            } else {
                None
            }
        };
        let _flags = Some(command[3].clone());
        let description = Some(command[4].clone());

        println!("server cmd");

        vec![
            Ts6Action::SetInfo(super::Ts6Info {
                sid,
                hopcount,
                description,
                name,

                identified: Some(true),
            }),
            Ts6Action::SendText(crate::sender::IrcResponse {
                sender: Some(hostname.to_owned().clone()),
                command: "SERVER".to_owned(),
                receiver: None,
                arguments: vec![
                    hostname.to_owned().clone(),
                    "1".to_owned(),
                    my_sid.clone().to_string(),
                    "+".to_owned(),
                ],
                message: String::from(":TODO"),
            }),
        ]
    }
}
