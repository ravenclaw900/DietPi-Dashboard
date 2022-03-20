use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(remote = "log::LevelFilter")]
pub enum LevelFilterWrap {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    #[serde(with = "LevelFilterWrap")]
    pub log_level: log::LevelFilter,

    pub port: u16,

    pub tls: bool,
    pub cert: String,
    pub key: String,

    pub pass: bool,
    pub hash: String,
    pub secret: String,
    pub expiry: u64,

    #[cfg(feature = "frontend")]
    pub nodes: Vec<String>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            log_level: log::LevelFilter::Info,

            port: 5252,

            tls: false,
            cert: String::new(),
            key: String::new(),

            pass: false,
            hash: String::new(),
            secret: String::new(),
            expiry: 3600,

            #[cfg(feature = "frontend")]
            nodes: Vec::new(),
        }
    }
}

pub fn config() -> Config {
    Figment::from(Serialized::defaults(Config::default()))
        .merge(Toml::file("config.toml"))
        .merge(Env::prefixed("DP_DASHBOARD_").ignore(&["hash", "secret"]))
        .extract()
        .expect("Error reading config")
}
