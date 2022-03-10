use std::str::FromStr;
use toml::Value;

pub struct Config {
    pub loglevel: log::LevelFilter,

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

pub fn config() -> Config {
    let mut cfgpath = std::env::current_exe().unwrap();
    cfgpath.set_file_name("config.toml");
    let cfg = &match std::fs::read_to_string(cfgpath) {
        Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
            std::fs::write("config.toml", "").unwrap();
            String::new()
        }
        Ok(cfg) => cfg,
        Err(e) => {
            panic!("Config file could not be read: {}", e);
        }
    }
    .parse::<Value>()
    .expect("Invalid config file");

    let loglevel = log::LevelFilter::from_str(
        cfg.get("loglevel")
            .unwrap_or_else(|| &Value::String("info".to_string()))
            .as_str()
            .unwrap(),
    )
    .unwrap();

    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    let port: u16 = cfg
        .get("port")
        .unwrap_or_else(|| &Value::Integer(5252))
        .as_integer()
        .unwrap() as u16;

    let tls = cfg
        .get("tls")
        .unwrap_or_else(|| &Value::Boolean(false))
        .as_bool()
        .unwrap();
    let mut cert = String::new();
    let mut key = String::new();
    if tls {
        cert = cfg
            .get("cert")
            .unwrap_or_else(|| &Value::String(String::new()))
            .as_str()
            .unwrap()
            .to_string();
        key = cfg
            .get("key")
            .unwrap_or_else(|| &Value::String(String::new()))
            .as_str()
            .unwrap()
            .to_string();
    }

    let pass = cfg
        .get("pass")
        .unwrap_or_else(|| &Value::Boolean(false))
        .as_bool()
        .unwrap();

    let mut hash = String::new();
    let mut secret = String::new();
    if pass {
        hash = cfg
            .get("hash")
            .unwrap_or_else(|| &Value::String(String::new()))
            .as_str()
            .unwrap()
            .to_string();
        secret = cfg
            .get("secret")
            .unwrap_or_else(|| &Value::String(String::new()))
            .as_str()
            .unwrap()
            .to_string();
    }

    #[allow(clippy::cast_sign_loss)]
    let expiry = cfg
        .get("expiry")
        .unwrap_or_else(|| &Value::Integer(3600))
        .as_integer()
        .unwrap() as u64;

    #[cfg(feature = "frontend")]
    let mut nodes = Vec::new();

    #[cfg(feature = "frontend")]
    for i in cfg
        .get("nodes")
        .unwrap_or_else(|| &Value::Array(Vec::new()))
        .as_array()
        .unwrap()
    {
        nodes.push(i.as_str().unwrap().to_string());
    }

    Config {
        loglevel,
        port,
        tls,
        cert,
        key,
        pass,
        hash,
        secret,
        expiry,
        #[cfg(feature = "frontend")]
        nodes,
    }
}
