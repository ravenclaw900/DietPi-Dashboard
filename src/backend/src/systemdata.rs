use anyhow::Context;
use once_cell::sync::Lazy;
use psutil::{cpu, disk, host, memory, network, process, sensors};
use std::fs;
use std::process::Command;
use std::str::from_utf8;
use std::sync::{
    atomic::{AtomicU64, Ordering::Relaxed},
    Mutex,
};
use std::time::Duration;
use tokio::time::sleep;

use crate::shared;

#[allow(clippy::cast_possible_truncation)]
fn round_percent(unrounded: f32) -> f32 {
    (unrounded * 100.0).round() / 100.0
}

pub async fn cpu() -> anyhow::Result<f32> {
    static CPU_COLLECTOR: Lazy<anyhow::Result<Mutex<cpu::CpuPercentCollector>>> =
        Lazy::new(|| Ok(Mutex::new(cpu::CpuPercentCollector::new()?)));

    sleep(Duration::from_secs(1)).await;
    Ok(round_percent(
        (CPU_COLLECTOR.as_ref())
            .map_err(|e| anyhow::anyhow!(e)) // Can't use context with an &Error
            .context("Couldn't init cpu collector")?
            .lock()
            .map_err(|e| anyhow::anyhow!(e.to_string())) // Mutex can't be sent between threads, so just get description of error
            .context("Couldn't lock cpu collector mutex")?
            .cpu_percent()
            .context("Couldn't get CPU percent")?,
    ))
}

pub fn ram() -> anyhow::Result<shared::UsageData> {
    let ram = memory::virtual_memory().context("Couldn't get memory data")?;

    Ok(shared::UsageData {
        used: ram.used(),
        total: ram.total(),
        percent: ram.percent(),
    })
}

pub fn swap() -> anyhow::Result<shared::UsageData> {
    let swap = memory::swap_memory().context("Couldn't get swap data")?;

    Ok(shared::UsageData {
        used: swap.used(),
        total: swap.total(),
        percent: swap.percent(),
    })
}

pub fn disk() -> anyhow::Result<shared::UsageData> {
    let disk = disk::disk_usage("/").context("Couldn't get disk usage data")?;

    Ok(shared::UsageData {
        used: disk.used(),
        total: disk.total(),
        percent: disk.percent(),
    })
}

pub fn network() -> anyhow::Result<shared::NetData> {
    static BYTES_SENT: AtomicU64 = AtomicU64::new(u64::MAX);
    static BYTES_RECV: AtomicU64 = AtomicU64::new(u64::MAX);

    static NET_COLLECTOR: Lazy<Mutex<network::NetIoCountersCollector>> =
        Lazy::new(|| Mutex::new(network::NetIoCountersCollector::default()));

    let network = NET_COLLECTOR
        .lock()
        .map_err(|e| anyhow::anyhow!(e.to_string())) // See CPU collector mutex
        .context("Couldn't lock network collector mutex")?
        .net_io_counters()
        .context("Couldn't get network data")?;
    let recv = network.bytes_recv();
    let sent = network.bytes_sent();

    let data = shared::NetData {
        received: recv.saturating_sub(BYTES_RECV.load(Relaxed)),
        sent: sent.saturating_sub(BYTES_SENT.load(Relaxed)),
    };

    BYTES_SENT.store(sent, Relaxed);
    BYTES_RECV.store(recv, Relaxed);

    Ok(data)
}

struct UnwrappedProcess {
    name: String,
    cmdline_exists: bool,
    cpu_percent: f32,
    ram: u64,
    status: psutil::process::Status,
}

// Single point of failure for fewer error statements
fn get_process_data(process: &mut psutil::process::Process) -> anyhow::Result<UnwrappedProcess> {
    Ok(UnwrappedProcess {
        name: process.name()?,
        cmdline_exists: process.cmdline()?.is_some(),
        cpu_percent: process.cpu_percent()?,
        ram: process.memory_info()?.rss(),
        status: process.status()?,
    })
}

// Processes may have changed, so don't return on error, just skip that process
pub async fn processes() -> anyhow::Result<Vec<shared::ProcessData>> {
    let mut processes = process::processes().context("Couldn't get list of processes")?;
    let mut process_list = Vec::new();
    process_list.reserve(processes.len());
    for element in &mut processes {
        match element.as_mut() {
            Ok(unwrapped_el) => match unwrapped_el.cpu_percent() {
                Ok(_) => (),
                Err(_) => continue,
            },
            Err(_) => continue,
        }
    }
    sleep(Duration::from_millis(500)).await;
    for mut element in processes.into_iter().flatten() {
        // Errors shouldn't return, just skip the process
        let process = if let Ok(process) = get_process_data(&mut element) {
            process
        } else {
            continue;
        };

        // Skip kernel threads
        if !process.cmdline_exists {
            continue;
        }
        let status = match process.status {
            // The processes that are running show up as sleeping, for some reason (or maybe I just misunderstand it)
            process::Status::Sleeping => "running",
            process::Status::Idle => "idle",
            process::Status::Stopped => "stopped",
            process::Status::Zombie => "zombie",
            process::Status::Dead => "dead",
            _ => "unknown",
        }
        .to_string();

        process_list.push(shared::ProcessData {
            pid: element.pid(),
            name: process.name,
            cpu: round_percent(process.cpu_percent),
            ram: process.ram,
            status,
        });
    }
    Ok(process_list)
}

// Return on error here, trust that DietPi-Software should work and if something goes wrong that it's bad
pub fn dpsoftware() -> anyhow::Result<(Vec<shared::DPSoftwareData>, Vec<shared::DPSoftwareData>)> {
    let free_out = Command::new("/boot/dietpi/dietpi-software")
        .arg("free")
        .output()
        .context("Couldn't get DietPi-Software free list")?
        .stdout;
    anyhow::ensure!(!free_out.is_empty(), "DietPi-Software not running as root");
    let free = from_utf8(&free_out)
        .context("Invalid DietPi-Software free list")?
        .lines()
        .nth(4)
        .context("DietPi-Software free list is too short")?
        .trim_start_matches("Free software ID(s): ");
    let free_list = if &free[..4] == "None" {
        Vec::new()
    } else {
        // Should be no negative software IDs, so ignore parsing errors by returning one
        free.split(' ')
            .map(|id| id.parse::<i16>().unwrap_or(-1))
            .collect()
    };

    let out = Command::new("/boot/dietpi/dietpi-software")
        .arg("list")
        .output()
        .context("Couldn't get DietPi-Software software list")?
        .stdout;
    let out_list = from_utf8(&out)
        .context("Invalid DietPi-Software software list")?
        .lines()
        .collect::<Vec<&str>>();
    let mut installed_list = Vec::new();
    let mut uninstalled_list = Vec::new();
    let mut index = 0_i16;
    uninstalled_list.reserve(
        out_list
            .len()
            .checked_sub(4) // Database messages
            .context("DietPi-Software software list is too short")?,
    );
    // First 4 skipped lines are the database messages
    'software: for element in out_list.iter().skip(4) {
        // Skip if in free list
        if free_list.contains(&(index as i16)) {
            index += 1;
        }
        let mut software = shared::DPSoftwareData::default();
        let mut installed = false;
        for (in1, el1) in element.split('|').enumerate() {
            match in1 {
                0 => {
                    software.id = el1
                        .trim()
                        .trim_start_matches("\u{001b}[32m")
                        .trim_start_matches("ID ")
                        .parse::<i16>()
                        .with_context(|| {
                            format!(
                                "Invalid software ID {}",
                                el1.trim()
                                    .trim_start_matches("\u{001b}[32m")
                                    .trim_start_matches("ID ")
                            )
                        })?;
                }
                1 => {
                    installed = el1
                        .trim()
                        .trim_start_matches('=')
                        .parse::<i8>()
                        .with_context(|| {
                            format!(
                                "Invalid installed specifier {} for id {}",
                                el1.trim().trim_start_matches('='),
                                software.id
                            )
                        })?
                        > 0;
                }
                2 => {
                    let mut name_desc = el1.trim().split(':');
                    software.name = name_desc
                        .next()
                        .with_context(|| {
                            format!("Couldn't get software name for id {}", software.id)
                        })?
                        .to_string();
                    software.description = name_desc
                        .next()
                        .with_context(|| {
                            format!("Couldn't get software description for id {}", software.id)
                        })?
                        .trim_start_matches("\u{001b}[0m \u{001b}[90m")
                        .trim_end_matches("\u{001b}[0m")
                        .to_string();
                }
                3 => {
                    // Annoying that here is the only place that software can be detected as disabled or not, and not before
                    if el1.contains("DISABLED") {
                        index += 1;
                        continue 'software;
                    }
                    software.dependencies = el1.trim().to_string();
                }
                4 => {
                    software.docs = el1
                        .trim()
                        .trim_start_matches("\u{001b}[90m")
                        .trim_end_matches("\u{001b}[0m")
                        .to_string();
                }
                _ => {}
            }
        }
        if installed {
            installed_list.push(software);
        } else {
            uninstalled_list.push(software);
        }
        index += 1;
    }
    Ok((uninstalled_list, installed_list))
}

pub fn host() -> anyhow::Result<shared::HostData> {
    let info = host::info();
    let uptime = host::uptime().context("Couldn't get uptime")?.as_secs() / 60;
    let dp_file = fs::read_to_string(&std::path::Path::new("/boot/dietpi/.version"))
        .context("Couldn't get DietPi version")?;
    let dp_version: Vec<&str> = dp_file.split(&['=', '\n'][..]).collect();
    // Much faster than 'apt list --installed'
    // Count number of newlines
    let installed_pkgs = Command::new("dpkg")
        .arg("--get-selections")
        .output()
        .context("Couldn't get number of installed APT packages")?
        .stdout
        .into_iter()
        .filter(|x| *x == b'\n')
        .count();
    let upgradable_pkgs = fs::read_to_string("/run/dietpi/.apt_updates")
        .unwrap_or_else(|_| 0u32.to_string())
        .trim_end_matches('\n')
        .parse::<u32>()
        .context("Couldn't parse number of APT updates")?;
    let mut arch = info.architecture().as_str();
    if arch == "unknown" {
        arch = "armv6/other";
    } else if arch == "arm" {
        arch = "armv7";
    }
    let addrs = &if_addrs::get_if_addrs().context("Couldn't get IP addresses")?;
    // Start with first address (probably loopback), and loop to try to get an actual one
    let mut addr = &addrs[0];
    for i in addrs {
        if !i.is_loopback() {
            addr = i;
            break;
        }
    }
    Ok(shared::HostData {
        hostname: info.hostname().to_string(),
        uptime,
        arch: arch.to_string(),
        kernel: info.release().to_string(),
        dp_version: format!("{}.{}.{}", dp_version[1], dp_version[3], dp_version[5]),
        packages: installed_pkgs,
        upgrades: upgradable_pkgs,
        ip: addr.ip().to_string(),
        nic: addr.name.clone(),
    })
}

// Also assume DietPi-Services output is good, and return on error
pub fn services() -> anyhow::Result<Vec<shared::ServiceData>> {
    let services = &mut Command::new("/boot/dietpi/dietpi-services")
        .arg("status")
        .output()
        .context("Couldn't get service list")?;
    anyhow::ensure!(
        !services.stdout.is_empty(),
        "DietPi-Services not running as root"
    );
    // Failures stored in stderr
    services.stdout.extend(&services.stderr);
    let services_str = from_utf8(&services.stdout).context("Invalid service list")?;
    let mut services_list = Vec::new();
    // Split on 3 different tokens
    for element in services_str
        .replace("[FAILED] DietPi-Services | \u{25cf} ", "dpdashboardtemp")
        .replace("[ INFO ] DietPi-Services | ", "dpdashboardtemp")
        .replace("[  OK  ] DietPi-Services | ", "dpdashboardtemp")
        .split("dpdashboardtemp")
        .skip(1)
    {
        let mut service = shared::ServiceData::default();
        // Only failed services
        if element.contains(".service") {
            for (index, el1) in element.split('\n').enumerate() {
                service.status = "failed".to_string();
                match index {
                    // Contains service, so shouldn't fail, but handle anyway
                    0 => {
                        service.name = el1
                            .split_once(".service")
                            .context("Couldn't get failed service name")?
                            .0
                            .to_string();
                    }
                    // Every line after 9 (before is data that's useless to us) should be service error log, format with HTML breaks
                    9.. => service.log.push_str(format!("{}<br>", el1).as_str()),
                    _ => (),
                }
            }
        } else {
            let (el1, el2) = match element.split_once('\t') {
                Some(els) => els,
                None => continue,
            };
            service.name = el1.trim().to_string();
            match el2.split_once(" since ") {
                Some(statusdate) => {
                    service.status = match statusdate.0.trim() {
                        "active (running)" | "active (exited)" => "active",
                        "inactive (dead)" => "inactive",
                        _ => "unknown",
                    }
                    .to_string();
                    service.start = statusdate.1.trim().to_string();
                }
                None => service.status = "inactive".to_string(),
            }
        }
        services_list.push(service);
    }
    Ok(services_list)
}

pub fn global() -> shared::GlobalData {
    use crate::CONFIG;

    let update =
        fs::read_to_string("/run/dietpi/.update_available").unwrap_or_else(|_| String::new());
    shared::GlobalData {
        update,
        login: CONFIG.pass,
        version: env!("CARGO_PKG_VERSION").to_string(),
        update_check: CONFIG.update_check,
        #[cfg(feature = "frontend")]
        nodes: CONFIG.nodes.clone(),
        temp_unit: CONFIG.temp_unit.clone(),
    }
}

pub fn browser_dir(path: &std::path::Path) -> anyhow::Result<Vec<shared::BrowserData>> {
    let dir =
        fs::read_dir(path).with_context(|| format!("Couldn't read path {}", path.display()))?;
    let mut file_list = Vec::new();
    for file in dir {
        let file = file.context("Couldn't get file data")?;
        // Resolve all symlinks
        let path = fs::canonicalize(file.path())
            .with_context(|| format!("Couldn't canonicalize path {}", file.path().display()))?;
        let metadata = fs::metadata(&path)
            .with_context(|| format!("Couldn't get metadata for path {}", &path.display()))?;
        let maintype;
        let subtype;
        let prettytype;
        if metadata.is_dir() {
            maintype = "dir".to_string();
            subtype = String::new();
            prettytype = "Directory".to_string();
        } else {
            let buf = fs::read(&path)
                .with_context(|| format!("Couldn't read directory {}", &path.display()))?;
            if let Some(infertype) = infer::get(&buf) {
                subtype = infertype
                    .mime_type()
                    .split_once('/')
                    .with_context(|| format!("Couldn't split mime type {}", infertype.mime_type()))?
                    .1
                    .to_string();
                maintype = {
                    if infer::is_app(&buf) {
                        "application"
                    } else if infer::is_archive(&buf) {
                        "archive"
                    } else if infer::is_audio(&buf) {
                        "audio"
                    } else if infer::is_image(&buf) {
                        "image"
                    } else if infer::is_video(&buf) {
                        "video"
                    } else {
                        "unknown"
                    }
                }
                .to_string();
                prettytype = format!(
                    "{} {}{} File",
                    subtype.to_uppercase(),
                    // Get first character, could (theoretically) panic
                    &maintype[0..1].to_uppercase(),
                    &maintype[1..]
                );
            } else if from_utf8(&buf).is_err() {
                maintype = "unknown".to_string();
                subtype = "unknown".to_string();
                prettytype = "Binary file".to_string();
            } else {
                if metadata.len() > 2 * 1000 * 1000 {
                    subtype = "large".to_string();
                } else {
                    subtype = "plain".to_string();
                }
                maintype = "text".to_string();
                prettytype = "Plain Text File".to_string();
            }
        }
        file_list.push(shared::BrowserData {
            path: crate::handle_error!(
                file.path()
                    .into_os_string()
                    .into_string()
                    .map_err(|_| anyhow::anyhow!("Invalid file path {}", file.path().display())),
                continue
            ),
            name: crate::handle_error!(
                file.file_name()
                    .into_string()
                    .map_err(|_| anyhow::anyhow!("Invalid file name {:?}", file.file_name())),
                continue
            ),
            maintype,
            subtype,
            prettytype,
            size: metadata.len(),
        });
    }
    Ok(file_list)
}

// Manually handle errors here, as there could just be no temperature sensor
#[allow(clippy::cast_possible_truncation)]
pub fn temp() -> shared::CPUTemp {
    match &sensors::temperatures().get(0) {
        Some(Ok(temp)) => {
            let temp = temp.current();
            shared::CPUTemp {
                available: true,
                celsius: temp.celsius().round() as i16,
                fahrenheit: temp.fahrenheit().round() as i16,
            }
        }
        None | Some(Err(_)) => shared::CPUTemp {
            available: false,
            celsius: 0,
            fahrenheit: 0,
        },
    }
}
