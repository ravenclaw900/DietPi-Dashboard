use anyhow::Context;
use psutil::{cpu, disk, host, memory, network, process, sensors};
use std::str::from_utf8;
use std::time::Duration;
use tokio::fs;
use tokio::process::Command;
use tokio::time::sleep;
use tracing::instrument;

use crate::shared;

fn round_percent(unrounded: f32) -> f32 {
    (unrounded * 100.0).round() / 100.0
}

#[instrument(skip_all)]
pub fn cpu(collector: &mut cpu::CpuPercentCollector) -> anyhow::Result<f32> {
    Ok(round_percent(
        collector
            .cpu_percent()
            .context("Couldn't get CPU percent")?,
    ))
}

#[instrument]
pub fn ram() -> anyhow::Result<shared::UsageData> {
    let ram = memory::virtual_memory().context("Couldn't get memory data")?;

    Ok(shared::UsageData {
        used: ram.used(),
        total: ram.total(),
        percent: ram.percent(),
    })
}

#[instrument]
pub fn swap() -> anyhow::Result<shared::UsageData> {
    let swap = memory::swap_memory().context("Couldn't get swap data")?;

    Ok(shared::UsageData {
        used: swap.used(),
        total: swap.total(),
        percent: swap.percent(),
    })
}

#[instrument]
pub fn disk() -> anyhow::Result<shared::UsageData> {
    let disk = disk::disk_usage("/").context("Couldn't get disk usage data")?;

    Ok(shared::UsageData {
        used: disk.used(),
        total: disk.total(),
        percent: disk.percent(),
    })
}

#[instrument(skip(collector))]
pub fn network(
    collector: &mut network::NetIoCountersCollector,
    prev_data: &mut shared::NetData,
) -> anyhow::Result<shared::NetData> {
    let network = collector
        .net_io_counters()
        .context("Couldn't get network data")?;
    let recv = network.bytes_recv();
    let sent = network.bytes_sent();

    let data = shared::NetData {
        received: recv.saturating_sub(prev_data.received),
        sent: sent.saturating_sub(prev_data.sent),
    };

    *prev_data = shared::NetData {
        received: recv,
        sent,
    };

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

#[instrument]
// Processes may have changed, so don't return on error, just skip that process
pub async fn processes() -> anyhow::Result<Vec<shared::ProcessData>> {
    let mut processes = process::processes().context("Couldn't get list of processes")?;
    let mut process_list = Vec::new();
    process_list.reserve(processes.len());
    for process in processes.iter_mut().flatten() {
        // Required to get cpu times before actual measurement
        if process.cpu_percent().is_err() {
            continue;
        }
    }
    sleep(Duration::from_millis(500)).await;
    for mut element in processes.into_iter().flatten() {
        // Errors shouldn't return, just skip the process
        let Ok(process) = get_process_data(&mut element) else {
            continue;
        };

        // Skip kernel threads
        if !process.cmdline_exists {
            continue;
        }
        let status = match process.status {
            process::Status::Sleeping => "sleeping",
            process::Status::Running => "running",
            process::Status::Idle => "idle",
            process::Status::Stopped => "stopped",
            process::Status::Zombie => "zombie",
            process::Status::Dead => "dead",
            _ => "unknown",
        };

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

#[instrument]
// Return on error here, trust that DietPi-Software should work and if something goes wrong that it's bad
pub async fn dpsoftware(
) -> anyhow::Result<(Vec<shared::DPSoftwareData>, Vec<shared::DPSoftwareData>)> {
    let out = Command::new("/boot/dietpi/dietpi-software")
        .args(["list", "--machine-readable"])
        .output()
        .await
        .context("Couldn't get DietPi-Software software list")?
        .stdout;
    anyhow::ensure!(!out.is_empty(), "DietPi-Software not running as root");
    let out_list = from_utf8(&out)
        .context("Invalid DietPi-Software software list")?
        .lines()
        .collect::<Vec<&str>>();
    let mut installed_list = Vec::new();
    let mut uninstalled_list = Vec::new();
    uninstalled_list.reserve(out_list.len());
    // First 4 skipped lines are the database messages
    'software: for element in out_list {
        let mut software = shared::DPSoftwareData::default();
        let mut installed = false;
        for (in1, el1) in element.split('|').enumerate() {
            match in1 {
                0 => {
                    if el1.contains("DISABLED") {
                        continue 'software;
                    }
                    software.id = el1
                        .parse::<i16>()
                        .with_context(|| format!("Invalid software ID {el1}"))?;
                }
                1 => {
                    installed = el1.parse::<i8>().with_context(|| {
                        format!("Invalid installed specifier {el1} for id {}", software.id)
                    })? > 0;
                }
                2 => software.name = el1.to_string(),
                3 => software.description = el1.to_string(),
                4 => software.dependencies = el1.replace(',', ", "),
                5 => software.docs = el1.to_string(),
                _ => {}
            }
        }
        if installed {
            installed_list.push(software);
        } else {
            uninstalled_list.push(software);
        }
    }
    Ok((uninstalled_list, installed_list))
}

#[instrument]
pub async fn host() -> anyhow::Result<shared::HostData> {
    let info = host::info();
    let uptime = host::uptime().context("Couldn't get uptime")?.as_secs() / 60;
    let dp_file = fs::read_to_string("/boot/dietpi/.version")
        .await
        .context("Couldn't get DietPi version")?;
    let dp_version: Vec<&str> = dp_file.split(['=', '\n']).collect();
    // Much faster than 'apt list --installed'
    // Count number of newlines
    let installed_pkgs = Command::new("dpkg")
        .arg("--get-selections")
        .output()
        .await
        .context("Couldn't get number of installed APT packages")?
        .stdout
        .into_iter()
        .filter(|x| *x == b'\n')
        .count();
    let upgradable_pkgs = fs::read_to_string("/run/dietpi/.apt_updates")
        .await
        .unwrap_or_else(|_| 0.to_string())
        .trim_end_matches('\n')
        .parse::<u32>()
        .context("Couldn't parse number of APT updates")?;
    let mut arch = info.architecture().as_str();
    if arch == "unknown" {
        arch = "armv6l/other";
    } else if arch == "arm" {
        arch = "armv7l";
    }
    let addrs = &if_addrs::get_if_addrs().context("Couldn't get IP addresses")?;
    let addr = addrs.iter().find(|x| !x.is_loopback()).unwrap_or(&addrs[0]);
    Ok(shared::HostData {
        hostname: info.hostname().to_string(),
        uptime,
        arch,
        kernel: info.release().to_string(),
        dp_version: format!("{}.{}.{}", dp_version[1], dp_version[3], dp_version[5]),
        packages: installed_pkgs,
        upgrades: upgradable_pkgs,
        ip: addr.ip().to_string(),
        nic: addr.name.clone(),
    })
}

#[instrument]
// Also assume DietPi-Services output is good, and return on error
pub async fn services() -> anyhow::Result<Vec<shared::ServiceData>> {
    let services = &mut Command::new("/boot/dietpi/dietpi-services")
        .arg("status")
        .output()
        .await
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
    for element in shared::remove_color_codes(services_str)
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
                service.status = "failed";
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
                    9.. => service.log.push_str(format!("{el1}<br>").as_str()),
                    _ => (),
                }
            }
        } else {
            let Some(els) = element.split_once('\t') else { continue };
            service.name = els.0.trim().to_string();
            match els.1.split_once(" since ") {
                Some(statusdate) => {
                    service.status = match statusdate.0.trim() {
                        "active (running)" | "active (exited)" => "active",
                        "inactive (dead)" => "inactive",
                        _ => "unknown",
                    };
                    service.start = statusdate.1.trim().to_string();
                }
                None => service.status = "inactive",
            }
        }
        services_list.push(service);
    }
    Ok(services_list)
}

#[instrument]
pub async fn global() -> shared::GlobalData {
    use crate::CONFIG;

    let update = fs::read_to_string("/run/dietpi/.update_available")
        .await
        .unwrap_or_default();
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

fn uppercase_first_letter(word: &str) -> (char, &str) {
    let first_letter = word.chars().next().unwrap().to_ascii_uppercase();
    (first_letter, &word[1..])
}

#[instrument]
pub async fn browser_dir(path: &std::path::Path) -> anyhow::Result<Vec<shared::BrowserData>> {
    let mut dir = fs::read_dir(path)
        .await
        .with_context(|| format!("Couldn't read path {}", path.display()))?;

    let mut file_list = Vec::new();

    while let Ok(Some(file)) = dir.next_entry().await {
        tracing::debug!(
            "Got {} with type {:?}",
            file.path().display(),
            file.file_type()
                .await
                .map(|x| {
                    if x.is_file() {
                        "file"
                    } else if x.is_dir() {
                        "directory"
                    } else if x.is_symlink() {
                        "symlink"
                    } else {
                        "something else"
                    }
                })
                .unwrap_or("unknown")
        );

        // Resolve all symlinks
        let path = fs::canonicalize(file.path())
            .await
            .with_context(|| format!("Couldn't canonicalize path {}", file.path().display()))?;
        let metadata = fs::metadata(&path)
            .await
            .with_context(|| format!("Couldn't get metadata for path {}", &path.display()))?;

        let maintype;
        let subtype;
        let prettytype;

        if metadata.is_dir() {
            maintype = "dir".to_string();
            subtype = String::new();
            prettytype = "Directory".to_string();
        } else if metadata.is_file() {
            let mime_type = mime_guess::from_path(&path).first_or_octet_stream();

            if mime_type == mime_guess::mime::APPLICATION_OCTET_STREAM {
                maintype = "unknown".to_string();
                subtype = "unknown".to_string();
                prettytype = "Binary file".to_string();
            } else if mime_type == mime_guess::mime::TEXT_STAR
                || mime_type == mime_guess::mime::APPLICATION_JAVASCRIPT // Javascript and JSON are also text files, for our purposes
                || mime_type == mime_guess::mime::APPLICATION_JSON
            {
                maintype = "text".to_string();
                subtype = mime_type.subtype().as_str().to_string();

                let subtype_upper = uppercase_first_letter(&subtype);

                prettytype = format!("{}{} File", subtype_upper.0, subtype_upper.1);
            } else {
                maintype = mime_type.type_().as_str().to_string();
                subtype = mime_type.subtype().as_str().to_string();

                let maintype_upper = uppercase_first_letter(&maintype);
                let subtype_upper = subtype.to_ascii_uppercase();

                prettytype = format!(
                    "{} {}{} File",
                    subtype_upper, maintype_upper.0, maintype_upper.1
                );
            }
        } else {
            maintype = "notafile".to_string();
            subtype = "notafile".to_string();
            prettytype = "Special File".to_string();
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

#[instrument]
// No error message, as there could just be no temperature sensor
#[allow(clippy::cast_possible_truncation)]
pub fn temp() -> shared::CPUTemp {
    let temps = sensors::temperatures();
    match &temps.get(
        // Prefer 'coretemp' sensor for Intel CPUs, otherwise fallback to first in list
        temps
            .iter()
            .filter_map(|x| x.as_ref().ok())
            .position(|x| x.unit() == "coretemp")
            .unwrap_or(0),
    ) {
        Some(Ok(temp)) => {
            let temp = temp.current();
            shared::CPUTemp {
                available: true,
                celsius: temp.celsius().round() as i16,
                fahrenheit: temp.fahrenheit().round() as i16,
            }
        }
        _ => shared::CPUTemp {
            available: false,
            celsius: 0,
            fahrenheit: 0,
        },
    }
}
