use anyhow::Context;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncRead, AsyncWrite};

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

pub fn remove_color_codes(s: &str) -> String {
    s.replace('\u{1b}', "")
        .replace("[33m", "")
        .replace("[90m", "")
        .replace("[0m", "")
        .replace("[32m", "")
        .replace("[31m", "")
        .replace("[38;5;154m", "")
        .replace("[J", "")
}

pub fn get_token_from_list<'a>(
    token_list: &'a str,
    pat: [char; 2],
    token_name: &str,
) -> Option<&'a str> {
    let mut iter = token_list.split(pat).map(str::trim);
    if !iter.any(|x| x == token_name) {
        return None;
    }
    if let Some(next) = iter.next() {
        return Some(next);
    }
    None
}

pub fn get_fingerprint(req: &hyper::Request<hyper::Body>) -> anyhow::Result<Option<String>> {
    let cookie = if let Some(cookie) = req.headers().get(hyper::header::COOKIE) {
        cookie.to_str().context("Invalid cookie list")?
    } else {
        return Ok(None);
    };
    let fingerprint_option = get_token_from_list(cookie, ['=', ';'], "FINGERPRINT");
    Ok(Some(hex::encode(ring::digest::digest(
        &ring::digest::SHA256,
        &hex::decode(if let Some(fingerprint) = fingerprint_option {
            fingerprint
        } else {
            return Ok(None);
        })
        .context("Invalid fingerprint token")?,
    ))))
}

pub struct HyperUpgradeAdaptor(pub hyper::upgrade::Upgraded);

impl smol::io::AsyncRead for HyperUpgradeAdaptor {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        let mut buf = tokio::io::ReadBuf::new(buf);
        smol::ready!(std::pin::Pin::new(&mut self.0).poll_read(cx, &mut buf))?;
        std::task::Poll::Ready(Ok(buf.filled().len()))
    }
}

impl smol::io::AsyncWrite for HyperUpgradeAdaptor {
    fn poll_write(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        std::pin::Pin::new(&mut self.0).poll_write(cx, buf)
    }

    fn poll_flush(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        std::pin::Pin::new(&mut self.0).poll_flush(cx)
    }

    fn poll_close(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        std::pin::Pin::new(&mut self.0).poll_shutdown(cx)
    }
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

#[derive(Deserialize)]
pub struct Request {
    #[serde(default)]
    page: Option<String>,
    #[serde(default)]
    cmd: Option<String>,
    #[serde(default)]
    args: Option<Vec<String>>,
    #[serde(default)]
    token: Option<String>,
}

#[derive(Debug)]
pub enum RequestTypes {
    Page(String),
    Cmd {
        cmd: String,
        args: Option<Vec<String>>,
    },
    Token(String),
}

impl TryFrom<Request> for RequestTypes {
    type Error = anyhow::Error;

    fn try_from(value: Request) -> Result<Self, Self::Error> {
        if let Some(page) = value.page {
            Ok(Self::Page(page))
        } else if let Some(cmd) = value.cmd {
            Ok(Self::Cmd {
                cmd,
                args: value.args,
            })
        } else if let Some(token) = value.token {
            Ok(Self::Token(token))
        } else {
            Err(anyhow::anyhow!("All fields are None"))
        }
    }
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
    pub fingerprint: String,
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
