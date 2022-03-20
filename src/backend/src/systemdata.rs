use lazy_static::lazy_static;
use psutil::{cpu, disk, host, memory, network, process};
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

lazy_static! {
    static ref CPUCOLLECTOR: Mutex<cpu::CpuPercentCollector> =
        Mutex::new(cpu::CpuPercentCollector::new().unwrap());
    static ref NETCOLLECTOR: Mutex<network::NetIoCountersCollector> =
        Mutex::new(network::NetIoCountersCollector::default());
    static ref BYTES_SENT: AtomicU64 = AtomicU64::new(
        NETCOLLECTOR
            .lock()
            .unwrap()
            .net_io_counters()
            .unwrap()
            .bytes_sent()
    );
    static ref BYTES_RECV: AtomicU64 = AtomicU64::new(
        NETCOLLECTOR
            .lock()
            .unwrap()
            .net_io_counters()
            .unwrap()
            .bytes_recv()
    );
}

#[allow(clippy::cast_possible_truncation)]
fn round_percent(unrounded: f32) -> f32 {
    (unrounded * 100.0).round() / 100.0
}

pub async fn cpu() -> f32 {
    sleep(Duration::from_secs(1)).await;
    round_percent(CPUCOLLECTOR.lock().unwrap().cpu_percent().unwrap())
}

pub fn ram() -> shared::UsageData {
    let ram = memory::virtual_memory().unwrap();

    shared::UsageData {
        used: ram.used(),
        total: ram.total(),
        percent: ram.percent(),
    }
}

pub fn swap() -> shared::UsageData {
    let swap = memory::swap_memory().unwrap();

    shared::UsageData {
        used: swap.used(),
        total: swap.total(),
        percent: swap.percent(),
    }
}

pub fn disk() -> shared::UsageData {
    let disk = disk::disk_usage("/").unwrap();

    shared::UsageData {
        used: disk.used(),
        total: disk.total(),
        percent: disk.percent(),
    }
}

pub fn network() -> shared::NetData {
    let network = NETCOLLECTOR.lock().unwrap().net_io_counters().unwrap();
    let recv = network.bytes_recv();
    let sent = network.bytes_sent();

    let data = shared::NetData {
        received: recv.saturating_sub(BYTES_RECV.load(Relaxed)),
        sent: sent.saturating_sub(BYTES_SENT.load(Relaxed)),
    };

    BYTES_SENT.store(sent, Relaxed);
    BYTES_RECV.store(recv, Relaxed);

    data
}

pub async fn processes() -> Vec<shared::ProcessData> {
    let mut processes = process::processes().unwrap();
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
    for element in processes {
        match element {
            Ok(mut unwrapped) => {
                // Everything could fail if the process terminates, if so skip the process
                let name = match unwrapped.name() {
                    Ok(unwrapped_name) => unwrapped_name,
                    Err(_) => continue,
                };
                if unwrapped.cmdline().unwrap().is_none() {
                    continue;
                }
                let status = match unwrapped.status().unwrap() {
                    // The processes that are running show up as sleeping, for some reason
                    process::Status::Sleeping => "running",
                    process::Status::Idle => "idle",
                    process::Status::Stopped => "stopped",
                    process::Status::Zombie => "zombie",
                    process::Status::Dead => "dead",
                    _ => "",
                }
                .to_string();
                process_list.push(shared::ProcessData {
                    pid: unwrapped.pid(),
                    name,
                    cpu: round_percent(unwrapped.cpu_percent().unwrap()),
                    ram: unwrapped.memory_info().unwrap().rss(),
                    status,
                });
            }
            Err(_) => continue,
        }
    }
    process_list
}

#[allow(clippy::too_many_lines)]
pub fn dpsoftware() -> (Vec<shared::DPSoftwareData>, Vec<shared::DPSoftwareData>) {
    let free_out = Command::new("/boot/dietpi/dietpi-software")
        .arg("free")
        .output()
        .unwrap()
        .stdout;
    let free = from_utf8(&free_out)
        .unwrap()
        .lines()
        .nth(4)
        .unwrap()
        .trim_start_matches("Free software ID(s): ");
    let free_list = if &free[..4] == "None" {
        Vec::new()
    } else {
        free.split(' ')
            .map(|id| id.parse::<i16>().unwrap())
            .collect()
    };
    let out = Command::new("/boot/dietpi/dietpi-software")
        .arg("list")
        .output()
        .unwrap()
        .stdout;
    let out_list = from_utf8(&out).unwrap().lines().collect::<Vec<&str>>();
    let mut installed_list = Vec::new();
    let mut uninstalled_list = Vec::new();
    let mut index = 0_i16;
    uninstalled_list.reserve(match out_list.len().checked_sub(9) {
        Some(num) => num,
        None => return (uninstalled_list, installed_list),
    });
    'software: for element in out_list.iter().skip(4).take(out_list.len() - 4) {
        if free_list.contains(&(index as i16)) {
            index += 1;
        }
        let mut id = 0;
        let mut installed = false;
        let mut name = String::new();
        let mut docs = String::new();
        let mut depends = String::new();
        let mut desc = String::new();
        for (in1, el1) in element.split('|').enumerate() {
            match in1 {
                0 => {
                    id = el1
                        .trim()
                        .trim_start_matches("\u{001b}[32m")
                        .trim_start_matches("ID ")
                        .parse::<i16>()
                        .unwrap();
                }
                1 => installed = el1.trim().trim_start_matches('=').parse::<i8>().unwrap() > 0,
                2 => {
                    let mut name_desc = el1.trim().split(':');
                    name = name_desc.next().unwrap().to_string();
                    desc = name_desc
                        .next()
                        .unwrap()
                        .trim_start_matches("\u{001b}[0m \u{001b}[90m")
                        .trim_end_matches("\u{001b}[0m")
                        .to_string();
                }
                3 => {
                    if el1.contains("DISABLED") {
                        index += 1;
                        continue 'software;
                    }
                    depends = el1.trim().to_string();
                }
                4 => {
                    docs = el1
                        .trim()
                        .trim_start_matches("\u{001b}[90m")
                        .trim_end_matches("\u{001b}[0m")
                        .to_string();
                }
                _ => {}
            }
        }
        if installed {
            installed_list.push(shared::DPSoftwareData {
                id,
                dependencies: depends,
                docs,
                name,
                description: desc,
            });
        } else {
            uninstalled_list.push(shared::DPSoftwareData {
                id,
                dependencies: depends,
                docs,
                name,
                description: desc,
            });
        }
        index += 1;
    }
    (uninstalled_list, installed_list)
}

pub fn host() -> shared::HostData {
    let info = host::info();
    let uptime = host::uptime().unwrap().as_secs() / 60;
    let dp_file = fs::read_to_string(&std::path::Path::new("/boot/dietpi/.version")).unwrap();
    let dp_version: Vec<&str> = dp_file.split(&['=', '\n'][..]).collect();
    let installed_pkgs = from_utf8(
        &Command::new("dpkg")
            .arg("--get-selections")
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap()
    .lines()
    .count();
    let upgradable_pkgs = fs::read_to_string("/run/dietpi/.apt_updates")
        .unwrap_or_else(|_| 0.to_string())
        .trim_end_matches('\n')
        .parse::<u32>()
        .unwrap();
    let mut arch = info.architecture().as_str();
    if arch == "unknown" {
        arch = "armv6/other";
    } else if arch == "arm" {
        arch = "armv7";
    }
    let addrs = &if_addrs::get_if_addrs().unwrap();
    let mut addr = &addrs[0];
    for i in addrs {
        if !i.is_loopback() {
            addr = i;
            break;
        }
    }
    shared::HostData {
        hostname: info.hostname().to_string(),
        uptime,
        arch: arch.to_string(),
        kernel: info.release().to_string(),
        version: format!("{}.{}.{}", dp_version[1], dp_version[3], dp_version[5]),
        packages: installed_pkgs,
        upgrades: upgradable_pkgs,
        ip: addr.ip().to_string(),
        nic: addr.name.clone(),
    }
}

pub fn services() -> Vec<shared::ServiceData> {
    let services = &Command::new("/boot/dietpi/dietpi-services")
        .arg("status")
        .output()
        .unwrap()
        .stdout;
    let services_str = from_utf8(services).unwrap();
    let mut services_list = Vec::new();
    for element in services_str
        .replace("[FAILED] DietPi-Services | \u{25cf} ", "dpdashboardtemp")
        .replace("[ INFO ] DietPi-Services | ", "dpdashboardtemp")
        .replace("[  OK  ] DietPi-Services | ", "dpdashboardtemp")
        .split("dpdashboardtemp")
        .skip(1)
    {
        let mut name = String::new();
        let mut log = String::new();
        let mut status = String::new();
        let mut start = String::new();
        if element.contains(".service") {
            for (index, el1) in element.split('\n').enumerate() {
                status = "failed".to_string();
                match index {
                    0 => name = el1.split_once(".service").unwrap().0.to_string(),
                    9.. => log.push_str(format!("{}<br>", el1).as_str()),
                    _ => (),
                }
            }
        } else {
            let (el1, el2) = element.split_once('\t').unwrap();
            name = el1.trim().to_string();
            match el2.split_once(" since ") {
                Some(statusdate) => {
                    match statusdate.0.trim() {
                        "active (running)" => status = "running".to_string(),
                        "active (exited)" => status = "exited".to_string(),
                        "inactive (dead)" => status = "dead".to_string(),
                        _ => status = "unknown".to_string(),
                    }
                    start = statusdate.1.trim().to_string();
                }
                None => status = "dead".to_string(),
            }
        }
        services_list.push(shared::ServiceData {
            name,
            log,
            status,
            start,
        });
    }
    services_list
}

pub fn global() -> shared::GlobalData {
    let update =
        fs::read_to_string("/run/dietpi/.update_available").unwrap_or_else(|_| String::new());
    shared::GlobalData {
        update,
        login: crate::CONFIG.pass,
        #[cfg(feature = "frontend")]
        nodes: crate::CONFIG.nodes.clone(),
    }
}

pub fn browser_dir(path: &std::path::Path) -> Vec<shared::BrowserDirData> {
    let dir = fs::read_dir(path).unwrap();
    let mut file_list = Vec::new();
    for file in dir {
        let file = file.unwrap();
        // Resolve all symlinks
        let path = fs::canonicalize(file.path()).unwrap();
        let metadata = fs::metadata(&path).unwrap();
        let maintype;
        let subtype;
        let prettytype;
        if metadata.is_dir() {
            maintype = "dir".to_string();
            subtype = String::new();
            prettytype = "Directory".to_string();
        } else {
            let buf = if let Ok(val) = fs::read(path) {
                val
            } else {
                log::error!("Could not read directory");
                return vec![shared::BrowserDirData {
                    path: "/".to_string(),
                    name: "ERROR".to_string(),
                    maintype: "dir".to_string(),
                    subtype: String::new(),
                    prettytype: "Could not read directory".to_string(),
                    size: 0,
                }];
            };
            if let Some(infertype) = infer::get(&buf) {
                subtype = infertype.mime_type().split_once('/').unwrap().1.to_string();
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
                    maintype.chars().next().unwrap().to_uppercase(),
                    &maintype[1..]
                );
            } else if from_utf8(&buf).is_err() {
                maintype = "unknown".to_string();
                subtype = "unknown".to_string();
                prettytype = "Binary file".to_string();
            } else {
                maintype = "text".to_string();
                subtype = "plain".to_string();
                prettytype = "Plain Text File".to_string();
            }
        }
        file_list.push(shared::BrowserDirData {
            path: file.path().into_os_string().into_string().unwrap(),
            name: file.file_name().into_string().unwrap(),
            maintype,
            subtype,
            prettytype,
            size: metadata.len(),
        });
    }
    file_list
}

#[allow(clippy::float_cmp)] // All it's doing is rounding, so there shouldn't be any floating point errors
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_round() {
        assert_eq!(round_percent(56.7396), 56.74);
        assert_eq!(round_percent(99.989), 99.99);
        assert_eq!(round_percent(99.999), 100.00);
        assert_eq!(round_percent(31.25), 31.25);
        assert_eq!(round_percent(20.323), 20.32);
        assert_eq!(round_percent(0.105), 0.11);
        assert_eq!(round_percent(0.001), 0.0);
    }

    #[tokio::test]
    async fn validate_cpu() {
        let output = cpu().await;
        assert!((0.0..=100.0).contains(&output));
    }

    #[allow(clippy::cast_precision_loss)]
    fn usage_test(used: u64, total: u64, percent_test: f32) {
        assert!(used <= total);
        if total != 0 {
            assert_eq!(
                round_percent((used as f32 / total as f32) * 100.0),
                percent_test
            );
        }
    }

    #[test]
    async fn validate_ram() {
        let output = ram();
        assert!(output.total > 0);
        usage_test(output.used, output.total, output.percent);
    }

    #[test]
    async fn validate_swap() {
        let output = swap();
        usage_test(output.used, output.total, output.percent);
    }

    // The disk function gets the percentage used from psutil, not calculated here, so we can't use usage_test
    #[tokio::test]
    async fn validate_disk() {
        let output = disk();
        assert!(output.used <= output.total);
    }

    #[tokio::test]
    async fn validate_network() {
        let mut output = network();
        assert_eq!(output.sent, 0);
        assert_eq!(output.received, 0);
        // Just make sure that it works
        for _ in 0..20 {
            sleep(Duration::from_millis(100)).await;
            let old_sent = BYTES_SENT.load(Relaxed);
            let old_recv = BYTES_RECV.load(Relaxed);
            output = network();
            assert_eq!(BYTES_SENT.load(Relaxed), output.sent + old_sent);
            assert_eq!(BYTES_RECV.load(Relaxed), output.received + old_recv);
        }
    }

    // Very little to actually validate here, just make sure that there are no errors
    #[tokio::test]
    async fn validate_processes() {
        for _ in 0..30 {
            processes().await;
        }
    }

    #[test]
    fn validate_software() {
        let output = dpsoftware();
        let cmd = Command::new("/boot/dietpi/dietpi-software")
            .arg("list")
            .output()
            .unwrap()
            .stdout;
        let mut install_counter = 0;
        let mut uninstall_counter = 0;
        for i in from_utf8(&cmd).unwrap().lines().skip(4) {
            if i.contains("DISABLED") {
                continue;
            }
            if i.split_once('|')
                .unwrap()
                .1
                .trim()
                .trim_start_matches('=')
                .starts_with('0')
            {
                uninstall_counter += 1;
            } else {
                install_counter += 1;
            }
        }
        assert_eq!(uninstall_counter, output.0.len());
        assert_eq!(install_counter, output.1.len());
    }

    #[test]
    fn validate_host() {
        let output = host();

        assert_eq!(
            output.kernel,
            from_utf8(&Command::new("uname").arg("-r").output().unwrap().stdout)
                .unwrap()
                .trim_end_matches('\n')
        );

        // The IP address shouldn't be the loopback
        assert_ne!(output.nic, "127.0.0.1");

        assert_eq!(
            output.hostname,
            from_utf8(&Command::new("hostname").output().unwrap().stdout)
                .unwrap()
                .trim_end_matches('\n')
        );
    }

    #[test]
    fn validate_services() {
        let output = services();
        for i in output {
            if i.status == "running" || i.status == "exited" {
                assert_ne!(i.start, "");
                assert_eq!(i.log, "");
            } else if i.status == "failed" {
                assert_ne!(i.log, "");
            }
        }
    }
}
