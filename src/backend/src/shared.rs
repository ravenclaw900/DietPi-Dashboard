use serde::{Deserialize, Serialize};

pub static CONFIG: once_cell::sync::Lazy<crate::config::Config> =
    once_cell::sync::Lazy::new(crate::config::config);

// Simple error handling macro, print out error and source (if available), and handle error if it exists
#[macro_export]
macro_rules! handle_error {
    ($e: expr $(, $handler:expr)?) => {
        match $e {
            Ok(val) => val,
            Err(err) => {
                tracing::warn!("{:#}", err);
                $($handler)?
            }
        }
    };
}

#[macro_export]
macro_rules! json_msg {
    ($e: expr, $handler:expr) => {
        Message::text(handle_error!(serde_json::to_string($e).context("Couldn't serialize json"), $handler))
    };
}

#[derive(Serialize, Default)]
pub struct SysData {
    pub cpu: f32,
    pub ram: UsageData,
    pub swap: UsageData,
    pub disk: UsageData,
    pub network: NetData,
    pub temp: CPUTemp,
}

#[derive(Serialize, Default)]
pub struct UsageData {
    pub used: u64,
    pub total: u64,
    pub percent: f32,
}

#[derive(Serialize, Default, Debug)]
pub struct NetData {
    pub sent: u64,
    pub received: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Request {
    #[serde(default)]
    pub page: String,
    #[serde(default)]
    pub cmd: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub token: String,
}

#[derive(Serialize)]
pub struct ProcessData {
    pub pid: u32,
    pub name: String,
    pub cpu: f32,
    pub ram: u64,
    pub status: &'static str,
}

#[derive(Serialize)]
pub struct ProcessList {
    pub processes: Vec<ProcessData>,
}

#[derive(Serialize, Default)]
pub struct DPSoftwareData {
    pub id: i16,
    pub name: String,
    pub description: String,
    pub dependencies: String,
    pub docs: String,
}

#[derive(Serialize, Default)]
pub struct DPSoftwareList {
    pub installed: Vec<DPSoftwareData>,
    pub uninstalled: Vec<DPSoftwareData>,
    pub response: String,
}

#[derive(Serialize, Default)]
pub struct HostData {
    pub hostname: String,
    pub uptime: u64,
    pub arch: &'static str,
    pub kernel: String,
    pub dp_version: String,
    pub packages: usize,
    pub upgrades: u32,
    pub nic: String,
    pub ip: String,
}

#[derive(Serialize, Default)]
pub struct ServiceData {
    pub name: String,
    pub log: String,
    pub status: &'static str,
    pub start: String,
}

#[derive(Serialize)]
pub struct ServiceList {
    pub services: Vec<ServiceData>,
}

#[derive(Serialize)]
pub struct GlobalData {
    pub update: String,
    pub version: String,
    pub login: bool,
    pub update_check: bool,
    #[cfg(feature = "frontend")]
    pub nodes: Vec<String>,
    pub temp_unit: TempUnit,
}

#[derive(Serialize)]
pub struct BrowserData {
    pub path: String,
    pub name: String,
    pub subtype: &'static str,
    pub maintype: &'static str,
    pub prettytype: String,
    pub size: u64,
}

#[derive(Serialize, Default)]
pub struct BrowserList {
    pub contents: Vec<BrowserData>,
}

#[derive(Serialize)]
pub struct TokenError {
    pub error: bool,
}

#[derive(Deserialize, Debug)]
pub struct FileRequest {
    #[serde(default)]
    pub cmd: String,
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub token: String,
    #[serde(default)]
    pub arg: String,
}

#[derive(Serialize)]
pub struct FileSize {
    pub size: usize,
}

#[derive(Serialize)]
pub struct FileUploadFinished {
    pub finished: bool,
}

#[derive(Serialize, Deserialize)]
pub struct JWTClaims {
    pub iss: String,
    pub exp: u64,
    pub iat: u64,
}

#[derive(Serialize, Default)]
pub struct CPUTemp {
    pub available: bool,
    pub celsius: i16,
    pub fahrenheit: i16,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TempUnit {
    Fahrenheit,
    Celsius,
}
