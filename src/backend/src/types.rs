#[derive(serde::Serialize)]
pub struct SysData {
    pub cpu: f32,
    pub ram: UsageData,
    pub swap: UsageData,
    pub disk: UsageData,
    pub network: NetData,
}

#[derive(serde::Serialize)]
pub struct UsageData {
    pub used: u64,
    pub total: u64,
    pub percent: f32,
}

#[derive(serde::Serialize)]
pub struct NetData {
    pub sent: u64,
    pub recieved: u64,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Request {
    #[serde(default)]
    pub page: String,
    #[serde(default)]
    pub cmd: String,
    #[serde(default)]
    pub args: Vec<String>,
}

#[derive(serde::Serialize)]
pub struct ProcessData {
    pub pid: i32,
    pub name: String,
    pub cpu: f32,
    pub ram: u64,
    pub status: String,
}

#[derive(serde::Serialize)]
pub struct ProcessList {
    pub processes: Vec<ProcessData>,
}

#[derive(serde::Serialize)]
pub struct DPSoftwareData {
    pub id: i16,
    pub installed: bool,
    pub name: String,
    pub description: String,
    pub dependencies: String,
    pub docs: String,
}

#[derive(serde::Serialize)]
pub struct DPSoftwareList {
    pub software: Vec<DPSoftwareData>,
    pub response: String,
}

#[derive(serde::Serialize)]
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

#[derive(serde::Serialize)]
pub struct ServiceData {
    pub name: String,
    pub log: String,
    pub status: String,
    pub start: String,
}

#[derive(serde::Serialize)]
pub struct ServiceList {
    pub services: Vec<ServiceData>,
}

#[derive(serde::Serialize)]
pub struct GlobalData {
    pub update: String,
}
