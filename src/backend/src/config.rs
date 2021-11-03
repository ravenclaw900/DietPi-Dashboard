use nanoserde::{Toml, TomlParser};

#[derive(Debug)]
pub struct Config {
    pub port: u16,
}

pub fn config() -> Config {
    let cfg = TomlParser::parse(&match std::fs::read_to_string("config.toml") {
        Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
            std::fs::write("config.toml", "").unwrap();
            String::new()
        }
        Ok(cfg) => cfg,
        Err(e) => {
            panic!("Config file could not be read: {}", e);
        }
    })
    .expect("Invalid config file");

    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    let port: u16 = cfg.get("port").unwrap_or(&Toml::Num(8080.0)).num() as u16;

    Config { port }
}
