use nanoserde::{DeJson, SerJson};

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
    pub recieved: u64,
}

#[derive(Debug, Clone, DeJson)]
pub struct Request {
    #[nserde(default)]
    pub page: String,
    #[nserde(default)]
    pub cmd: String,
    #[nserde(default)]
    pub args: Vec<String>,
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
    pub installed: bool,
    pub name: String,
    pub description: String,
    pub dependencies: String,
    pub docs: String,
}

#[derive(SerJson)]
pub struct DPSoftwareList {
    pub software: Vec<DPSoftwareData>,
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
}

#[derive(SerJson, Debug)]
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
    pub data: String,
    pub currentpath: String,
}

#[derive(SerJson)]
pub struct BrowserList {
    pub contents: Vec<BrowserDirData>,
    pub currentpath: String,
}
