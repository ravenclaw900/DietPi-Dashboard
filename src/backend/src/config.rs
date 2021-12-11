use toml::Value;

pub struct Config {
    pub port: u16,

    pub tls: bool,
    pub cert: String,
    pub key: String,

    pub pass: bool,
    pub hash: String,
    pub secret: String,
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

    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    let port: u16 = cfg
        .get("port")
        .unwrap_or(&Value::Integer(5252))
        .as_integer()
        .unwrap() as u16;

    let tls = cfg
        .get("tls")
        .unwrap_or(&Value::Boolean(false))
        .as_bool()
        .unwrap();
    let mut cert = String::new();
    let mut key = String::new();
    if tls {
        cert = cfg
            .get("cert")
            .unwrap_or(&Value::String(String::new()))
            .as_str()
            .unwrap()
            .to_string();
        key = cfg
            .get("key")
            .unwrap_or(&Value::String(String::new()))
            .as_str()
            .unwrap()
            .to_string();
    }

    let pass = cfg
        .get("pass")
        .unwrap_or(&Value::Boolean(false))
        .as_bool()
        .unwrap();

    let mut hash = String::new();
    let mut secret = String::new();
    if pass {
        hash = cfg
            .get("hash")
            .unwrap_or(&Value::String(String::new()))
            .as_str()
            .unwrap()
            .to_string();
        secret = cfg
            .get("secret")
            .unwrap_or(&Value::String(String::new()))
            .as_str()
            .unwrap()
            .to_string();
    }

    let mut nodes = Vec::new();

    for i in cfg
        .get("nodes")
        .unwrap_or(&Value::Array(Vec::new()))
        .as_array()
        .unwrap()
    {
        nodes.push(i.as_str().unwrap().to_string());
    }

    dbg!(&nodes);

    Config {
        port,
        tls,
        cert,
        key,
        pass,
        hash,
        secret,
        nodes,
    }
}
