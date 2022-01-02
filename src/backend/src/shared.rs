use nanoserde::{DeJson, SerJson};

lazy_static::lazy_static! {
    pub static ref CONFIG: crate::config::Config = crate::config::config();
}

pub fn validate_token(token: &str) -> bool {
    let key = jwts::jws::Key::new(&crate::CONFIG.secret, jwts::jws::Algorithm::HS256);
    let verified: jwts::jws::Token<jwts::Claims>;
    if let Ok(token) = jwts::jws::Token::verify_with_key(token, &key) {
        verified = token;
    } else {
        log::error!("Couldn't verify token");
        return false;
    };
    let config = jwts::ValidationConfig {
        iat_validation: false,
        nbf_validation: false,
        exp_validation: true,
        expected_iss: Some("DietPi Dashboard".to_string()),
        expected_sub: None,
        expected_aud: None,
        expected_jti: None,
    };
    if verified.validate_claims(&config).is_err() {
        return false;
    }
    true
}

#[derive(SerJson)]
pub struct SysData {
    pub cpu: f32,
    pub ram: UsageData,
    pub swap: UsageData,
    pub disk: UsageData,
    pub network: NetData,
}

#[derive(SerJson)]
pub struct UsageData {
    pub used: u64,
    pub total: u64,
    pub percent: f32,
}

#[derive(SerJson)]
pub struct NetData {
    pub sent: u64,
    pub received: u64,
}

#[derive(Debug, Clone, DeJson)]
pub struct Request {
    #[nserde(default)]
    pub page: String,
    #[nserde(default)]
    pub cmd: String,
    #[nserde(default)]
    pub args: Vec<String>,
    #[nserde(default)]
    pub token: String,
}

#[derive(SerJson)]
pub struct ProcessData {
    pub pid: i32,
    pub name: String,
    pub cpu: f32,
    pub ram: u64,
    pub status: String,
}

#[derive(SerJson)]
pub struct ProcessList {
    pub processes: Vec<ProcessData>,
}

#[derive(SerJson)]
pub struct DPSoftwareData {
    pub id: i16,
    pub name: String,
    pub description: String,
    pub dependencies: String,
    pub docs: String,
}

#[derive(SerJson)]
pub struct DPSoftwareList {
    pub installed: Vec<DPSoftwareData>,
    pub uninstalled: Vec<DPSoftwareData>,
    pub response: String,
}

#[derive(SerJson)]
pub struct HostData {
    pub hostname: String,
    pub uptime: u64,
    pub arch: String,
    pub kernel: String,
    pub version: String,
    pub packages: usize,
    pub upgrades: u32,
    pub nic: String,
    pub ip: String,
}

#[derive(SerJson)]
pub struct ServiceData {
    pub name: String,
    pub log: String,
    pub status: String,
    pub start: String,
}

#[derive(SerJson)]
pub struct ServiceList {
    pub services: Vec<ServiceData>,
}

#[derive(SerJson)]
pub struct GlobalData {
    pub update: String,
    pub login: bool,
    #[cfg(feature = "frontend")]
    pub nodes: Vec<String>,
}

#[derive(SerJson)]
pub struct BrowserDirData {
    pub path: String,
    pub name: String,
    pub subtype: String,
    pub maintype: String,
    pub prettytype: String,
    pub size: u64,
}

#[derive(SerJson)]
pub struct BrowserFileData {
    pub textdata: String,
}

#[derive(SerJson)]
pub struct BrowserList {
    pub contents: Vec<BrowserDirData>,
}

#[derive(SerJson)]
pub struct TokenError {
    pub error: bool,
}
