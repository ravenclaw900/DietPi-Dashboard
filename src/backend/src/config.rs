use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use log::LevelFilter;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(remote = "LevelFilter")]
#[serde(rename_all = "lowercase")]
enum LevelFilterDef {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TempUnit {
    Fahrenheit,
    Celsius,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    #[serde(with = "LevelFilterDef")]
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

    pub terminal_user: String,

    pub update_check: bool,

    pub temp_unit: TempUnit,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            log_level: LevelFilter::Info,

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

            terminal_user: "root".to_string(),

            update_check: true,

            temp_unit: TempUnit::Celsius,
        }
    }
}

pub fn config() -> Config {
    let mut cfgpath = std::env::current_exe().expect("Couldn't get config path");
    cfgpath.set_file_name("config.toml");
    Figment::from(Serialized::defaults(Config::default()))
        .merge(Toml::file(cfgpath))
        .merge(Env::prefixed("DP_DASHBOARD_").ignore(&["hash", "secret"]))
        .extract()
        .expect("Error reading config")
}
