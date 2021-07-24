use serde_derive::{Serialize, Deserialize};
use std::fs::read_to_string;

#[derive(Clone)]
pub struct ConcreteConfig {
    pub server_port: u16,
    pub server_motd: String,
    pub networking_enable_compression: bool,
    pub networking_online_mode: bool
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub server: Option<ConfigServer>
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ConfigServer {
    pub port: Option<u16>,
    pub motd: Option<String>,
    pub networking: Option<ConfigServerNetworking>
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ConfigServerNetworking {
    pub enable_compression: Option<bool>,
    pub online_mode: Option<bool>
}

impl Config {
    pub fn load() -> Config {
        toml::from_str(read_to_string("config.toml").unwrap().as_str()).unwrap()
    }
}

impl From<Config> for ConcreteConfig {
    fn from(conf: Config) -> Self {
        let mut c = Self {
            server_port: 25565,
            server_motd: String::from("Hello, World!"),
            networking_enable_compression: true,
            networking_online_mode: true
        };

        if let Some(server) = conf.server {
            if let Some(v) = server.port { c.server_port = v; }
            if let Some(v) = server.motd { c.server_motd = v; }
        }

        c
    }
}
