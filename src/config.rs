use std::path::{Path, PathBuf};

use crate::shared::TempUnit;
use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};

#[cfg(feature = "frontend")]
const CONFIG_FILE: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config-frontend.toml"));
#[cfg(not(feature = "frontend"))]
const CONFIG_FILE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config-backend.toml"));

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub log_level: String,

    pub port: u16,

    pub tls: bool,
    pub cert: PathBuf,
    pub key: PathBuf,

    #[cfg(feature = "frontend")]
    pub pass: bool,
    #[cfg(feature = "frontend")]
    pub hash: String,
    #[cfg(feature = "frontend")]
    pub priv_key: PathBuf,
    pub pub_key: PathBuf,
    #[cfg(feature = "frontend")]
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

            #[cfg(feature = "frontend")]
            pass: false,
            #[cfg(feature = "frontend")]
            hash: String::new(),
            #[cfg(feature = "frontend")]
            priv_key: PathBuf::new(),
            pub_key: PathBuf::new(),
            #[cfg(feature = "frontend")]
            expiry: 3600,

            #[cfg(feature = "frontend")]
            nodes: Vec::new(),

            terminal_user: "root".to_string(),

            update_check: true,

            temp_unit: TempUnit::Celsius,
        }
    }
}

fn replace_relative_path(file_name: &Path) -> Option<PathBuf> {
    if file_name.is_relative() && !file_name.as_os_str().is_empty() {
        return Some(
            std::env::current_exe()
                .expect("Couldn't get executable path")
                .with_file_name(file_name),
        );
    }
    None
}

pub fn config() -> Config {
    // Guaranteed to be a relative path
    let cfgpath = replace_relative_path(Path::new("config.toml")).unwrap();
    if !cfgpath.exists() {
        std::fs::write(&cfgpath, CONFIG_FILE).expect("Couldn't write config file");
    }
    let mut config: Config = Figment::from(Serialized::defaults(Config::default()))
        .merge(Toml::file(cfgpath))
        .merge(Env::prefixed("DP_DASHBOARD_").ignore(&["hash", "priv_key", "pub_key"]))
        .extract()
        .expect("Error reading config");
    if config.tls {
        config.cert = replace_relative_path(&config.cert).unwrap_or(config.cert);
        config.key = replace_relative_path(&config.key).unwrap_or(config.key);
    }
    #[cfg(feature = "frontend")]
    {
        config.priv_key = replace_relative_path(&config.priv_key).unwrap_or(config.priv_key);
    }
    config.pub_key = replace_relative_path(&config.pub_key).unwrap_or(config.pub_key);
    config
}
