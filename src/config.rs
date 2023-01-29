use std::path::PathBuf;

use crate::shared::TempUnit;
use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};

#[cfg(feature = "frontend")]
const CONFIG_FILE: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config.toml"));
#[cfg(not(feature = "frontend"))]
const CONFIG_FILE: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config.toml"));

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub log_level: String,

    pub port: u16,

    pub tls: bool,
    pub cert: PathBuf,
    pub key: PathBuf,

    pub pass: bool,
    pub hash: String,
    #[cfg(feature = "frontend")]
    pub priv_key: PathBuf,
    #[cfg(not(feature = "frontend"))]
    pub pub_key: PathBuf,
    pub expiry: u64,

    #[cfg(feature = "frontend")]
    pub nodes: Vec<String>,

    pub terminal_user: String,

    pub update_check: bool,

    pub temp_unit: TempUnit,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            log_level: "info".to_string(),

            port: 5252,

            tls: false,
            cert: PathBuf::new(),
            key: PathBuf::new(),

            pass: false,
            hash: String::new(),
            #[cfg(feature = "frontend")]
            priv_key: PathBuf::new(),
            #[cfg(not(feature = "frontend"))]
            pub_key: PathBuf::new(),
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
    if !cfgpath.exists() {
        std::fs::write(&cfgpath, CONFIG_FILE).expect("Couldn't write config file");
    }
    Figment::from(Serialized::defaults(Config::default()))
        .merge(Toml::file(cfgpath))
        .merge(Env::prefixed("DP_DASHBOARD_").ignore(&["hash", "secret"]))
        .extract()
        .expect("Error reading config")
}
