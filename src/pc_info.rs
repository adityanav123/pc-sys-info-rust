use std::{ffi::OsString, net::IpAddr};

use serde::{Deserialize, Serialize};
use sysinfo::{Process, System};

#[derive(Serialize, Deserialize, Debug)]
pub struct PcInfo {
    ram_bytes: u64,
    cpu_cores: u8,
    total_ram_bytes: u64,
    top_five_processes: Vec<(String, u64)>, // top 5 heavy processes running.
    ip_addr: String,
    hostname: String,
}

impl PcInfo {
    pub fn new() -> Self {
        Self {
            ram_bytes: 0,
            cpu_cores: 0,
            total_ram_bytes: 0,
            top_five_processes: Vec::new(),
            ip_addr: String::new(),
            hostname: String::new(),
        }
    }

    pub fn fetch_data(&mut self, system: &mut System) {
        system.refresh_all();

        self.ram_bytes = system.used_memory();
        self.total_ram_bytes = system.total_memory();
        self.cpu_cores = system.cpus().len() as u8;

        let mut processes: Vec<&Process> = system.processes().values().collect();
        processes.sort_by(|a, b| b.memory().cmp(&a.memory()));

        self.top_five_processes = processes
            .into_iter()
            .take(5)
            .map(|process| (get_proc_name(process), process.memory()))
            .collect();

        self.ip_addr = get_ip_addr();
        self.hostname = get_hostname();
    }

    pub fn hostname(&self) -> &str {
        &self.hostname
    }
    pub fn ip_addr(&self) -> &str {
        &self.ip_addr
    }
    pub fn ram_bytes(&self) -> u64 {
        self.ram_bytes
    }
    pub fn total_ram_bytes(&self) -> u64 {
        self.total_ram_bytes
    }
    pub fn cpu_cores(&self) -> u8 {
        self.cpu_cores
    }
    pub fn top_five_processes(&self) -> &[(String, u64)] {
        &self.top_five_processes
    }

    // JSON convert
    pub fn to_json(&self) -> String {
        // using serde
        let json_obj = serde_json::to_string_pretty(&self);
        match json_obj {
            Ok(obj) => obj,
            Err(_) => "".to_string(),
        }
    }
}

pub fn get_proc_name(process: &Process) -> String {
    process.name().to_string_lossy().into_owned()
}

pub fn get_hostname() -> String {
    hostname::get()
        .unwrap_or_else(|_| OsString::from("unknown"))
        .to_string_lossy()
        .into_owned()
}

pub fn get_ip_addr() -> String {
    match local_ip_address::local_ip() {
        Ok(IpAddr::V4(ip)) => ip.to_string(),
        Ok(IpAddr::V6(ip)) => ip.to_string(),
        Err(_) => "0.0.0.0".to_string(),
    }
}
