use std::{env::home_dir, fs::read_to_string, path::PathBuf};

use crate::error_structs::ConfigReadError;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize)]
pub struct ServerInfo {
    pub ip: String,
    pub port: u64,
    pub server_hostname: String,
    pub network_name: String,
    pub operators: Vec<String>,
    pub server_incoming_passwords: Vec<String>,
    pub server_outgoing_password: String,
}

fn get_config_path() -> Result<PathBuf, ConfigReadError> {
    if cfg!(target_os = "linux") {
        if let Some(mut homedir) = home_dir() {
            homedir.push(".config");
            homedir.push("irs");
            homedir.push("config.toml");

            if homedir.exists() {
                return Ok(homedir);
            }
        }

        let dir = PathBuf::from("/etc/irs/config.toml");
        if dir.exists() {
            dir
        } else {
            return Err(ConfigReadError::NoConfigFile);
        }
    } else {
        return Err(ConfigReadError::UnsupportedOS);
    };

    unreachable!()
}

impl ServerInfo {
    pub fn load(path: Option<String>) -> Result<Self, ConfigReadError> {
        let path = if let Some(path) = path {
            PathBuf::from(path)
        } else {
            get_config_path()?
        };
        let config: ServerInfo = toml::from_str(&read_to_string(path)?)?;

        Ok(config)
    }
}
