use lazy_static::lazy_static;
use psutil::{cpu, disk, host, memory, network, process};
use std::fs;
use std::sync::Mutex;
use std::{process::Command, thread, time};

use crate::types;

lazy_static! {
    static ref CPUCOLLECTOR: Mutex<cpu::CpuPercentCollector> =
        Mutex::new(cpu::CpuPercentCollector::new().unwrap());
    static ref NETCOLLECTOR: Mutex<network::NetIoCountersCollector> =
        Mutex::new(network::NetIoCountersCollector::default());
}

pub fn cpu() -> f32 {
    thread::sleep(time::Duration::from_millis(500));
    (CPUCOLLECTOR.lock().unwrap().cpu_percent().unwrap() * 100.0).round() / 100.0
}

pub fn ram() -> types::UsageData {
    let ram = memory::virtual_memory().unwrap();

    types::UsageData {
        used: ram.used(),
        total: ram.total(),
        percent: ram.percent(),
    }
}

pub fn swap() -> types::UsageData {
    let swap = memory::swap_memory().unwrap();

    types::UsageData {
        used: swap.used(),
        total: swap.total(),
        percent: swap.percent(),
    }
}

pub fn disk() -> types::UsageData {
    let disk = disk::disk_usage("/").unwrap();

    types::UsageData {
        used: disk.used(),
        total: disk.total(),
        percent: disk.percent(),
    }
}

pub fn network() -> types::NetData {
    let network = NETCOLLECTOR.lock().unwrap().net_io_counters().unwrap();

    return types::NetData {
        recieved: network.bytes_recv(),
        sent: network.bytes_sent(),
    };
}

pub fn processes() -> Vec<types::ProcessData> {
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
    thread::sleep(time::Duration::from_millis(500));
    for element in processes {
        let mut unwrapped;
        match element {
            Ok(unwrapped_el) => unwrapped = unwrapped_el,
            Err(_) => continue,
        }
        // Name could fail if the process terminates, if so skip the process
        let name;
        match unwrapped.name() {
            Ok(unwrapped_name) => name = unwrapped_name,
            Err(_) => continue,
        }
        let status: String;
        match unwrapped.status().unwrap() {
            // The proceses that are running show up as sleeping, for some reason
            process::Status::Sleeping => status = "running".to_string(),
            process::Status::Idle => status = "idle".to_string(),
            process::Status::Stopped => status = "stopped".to_string(),
            process::Status::Zombie => status = "zombie".to_string(),
            process::Status::Dead => status = "dead".to_string(),
            _ => status = String::new(),
        }
        process_list.push(types::ProcessData {
            pid: unwrapped.pid(),
            name,
            cpu: (unwrapped.cpu_percent().unwrap() * 100.0).round() / 100.0,
            ram: unwrapped.memory_info().unwrap().vms() / 1_048_576,
            status,
        });
    }
    process_list
}

pub fn dpsoftware() -> Vec<types::DPSoftwareData> {
    let out = Command::new("/boot/dietpi/dietpi-software")
        .args(["list"])
        .output()
        .unwrap()
        .stdout;
    let out_list = std::str::from_utf8(out.as_slice())
        .unwrap()
        .split('\n')
        .collect::<Vec<&str>>();
    let mut software_list = Vec::new();
    software_list.reserve(out_list.len() - 9);
    'software: for element in out_list.iter().skip(4).take(out_list.len() - 5) {
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
                        .parse::<i32>()
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
                        software_list.push(types::DPSoftwareData {
                            id: -1,
                            installed: false,
                            name: String::new(),
                            description: String::new(),
                            dependencies: String::new(),
                            docs: String::new(),
                        });
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
        software_list.push(types::DPSoftwareData {
            id,
            dependencies: depends,
            docs,
            name,
            description: desc,
            installed,
        });
    }
    software_list
}

pub fn host() -> types::HostData {
    let info = host::info();
    let uptime = host::uptime().unwrap().as_secs();
    let dp_file = fs::read_to_string(&std::path::Path::new("/boot/dietpi/.version")).unwrap();
    let dp_version: Vec<&str> = dp_file.split(&['=', '\n'][..]).collect();
    types::HostData {
        hostname: info.hostname().to_string(),
        uptime,
        arch: info.architecture().as_str().to_string(),
        kernel: info.release().to_string(),
        version: format!("{}.{}.{}", dp_version[1], dp_version[3], dp_version[5]),
    }
}
