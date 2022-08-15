use std::io::{Read, Seek, Write};

use crate::shared::TempUnit;
use anyhow::Context;
use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};

const CURRENT_CONFIG_VERSION: u16 = 1;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub version: u16,

    pub log_level: String,

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
            version: 0,

            log_level: "info".to_string(),

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

fn migrate_config(path: &std::path::Path, cfg_version: u16) -> anyhow::Result<Config> {
    // Should tokio be used here?
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .read(true)
        .open(path)
        .context("Couldn't open TOML file")?;
    let mut file_data = String::new();
    file.read_to_string(&mut file_data)
        .context("Couldn't read TOML file")?;

    file.seek(std::io::SeekFrom::Start(0))
        .context("Couldn't seek back to beginning of file")?;

    let mut toml = file_data
        .parse::<toml_edit::Document>()
        .expect("Invalid config file");

    for i in (0..=CURRENT_CONFIG_VERSION).filter(|x| *x > cfg_version) {
        match i {
            1 => {
                toml.insert("temp_unit", toml_edit::value("celsius"));
            }
            _ => unreachable!(),
        }
        toml.insert("version", toml_edit::value(i64::from(i)));
    }

    let toml_string = toml.to_string();
    file.write_all(toml_string.as_bytes())
        .context("Couldn't update TOML file")?;
    Ok(Figment::from(Serialized::defaults(Config::default()))
        .merge(Toml::string(&toml_string))
        .merge(Env::prefixed("DP_DASHBOARD_").ignore(&["hash", "secret", "version"]))
        .extract()
        .expect("Error reading config"))
}

pub fn config() -> Config {
    let cfgpath = std::env::current_exe()
        .expect("Couldn't get config path")
        .with_file_name("config.toml");
    let cfg: Config = Figment::from(Serialized::defaults(Config::default()))
        .merge(Toml::file(&cfgpath))
        .merge(Env::prefixed("DP_DASHBOARD_").ignore(&["hash", "secret", "version"]))
        .extract()
        .expect("Error reading config");
    if cfg.version < CURRENT_CONFIG_VERSION {
        crate::handle_error!(migrate_config(&cfgpath, cfg.version), cfg)
    } else {
        cfg
    }
}
