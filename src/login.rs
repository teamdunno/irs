use tokio::{io::BufWriter, net::TcpStream};

use crate::{ServerInfo, error_structs::SenderError, sender::IrcResponseCodes, user::User};

pub async fn send_motd(
    server_info: ServerInfo,
    user_info: User,
    writer: &mut BufWriter<TcpStream>,
) -> Result<(), SenderError> {
    let user_info = user_info.unwrap_all();
    let server_version = &format!("IRS-v{}", env!("CARGO_PKG_VERSION")) as &str;

    let welcome_text = format!(
        "Welcome to the {} Internet Relay Chat Network {}",
        server_info.network_name, user_info.nickname
    );
    let yourhost_text = format!(
        "Your host is {}, running version {}",
        server_info.server_hostname, server_version
    );
    let myinfo_text = format!("{} {} i b", server_info.server_hostname, server_version);
    let isupport_text = format!(
        "CHANTYPES=# NETWORK={} :are supported by this server",
        server_info.network_name
    );

    IrcResponseCodes::Welcome
        .into_irc_response(user_info.nickname.clone(), welcome_text)
        .send(&server_info.server_hostname, writer, true)
        .await?;
    IrcResponseCodes::YourHost
        .into_irc_response(user_info.nickname.clone(), yourhost_text)
        .send(&server_info.server_hostname, writer, true)
        .await?;
    IrcResponseCodes::MyInfo
        .into_irc_response(user_info.nickname.clone(), myinfo_text)
        .send(&server_info.server_hostname, writer, false)
        .await?;
    IrcResponseCodes::ISupport
        .into_irc_response(user_info.nickname.clone(), isupport_text)
        .send(&server_info.server_hostname, writer, false)
        .await?;
    IrcResponseCodes::NoMotd
        .into_irc_response(
            user_info.username.clone(),
            "MOTD not implemented yet".into(),
        )
        .send(&server_info.server_hostname, writer, true)
        .await?;

    Ok(())
}
