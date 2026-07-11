use hark_core::model::{CpuInfo, DiskInfo, MemoryInfo, NetworkInfo, ProcessInfo, SystemSnapshot};
use sysinfo::{Disks, Networks, System};

pub struct Collector {
    system: System,
    networks: Networks,
    disks: Disks,
}

impl Collector {
    pub fn new() -> Self {
        Self {
            system: System::new_all(),
            networks: Networks::new_with_refreshed_list(),
            disks: Disks::new_with_refreshed_list(),
        }
    }

    pub fn snapshot(&mut self) -> SystemSnapshot {
        self.system.refresh_all();
        self.networks.refresh(true);
        self.disks.refresh(true);

        SystemSnapshot {
            cpu: self.collect_cpu(),
            memory: self.collect_memory(),
            disks: self.collect_disks(),
            networks: self.collect_networks(),
            processes: self.collect_processes(),
        }
    }

    fn collect_cpu(&self) -> CpuInfo {
        CpuInfo {
            global_usage: self.system.global_cpu_usage(),
            cores: self.system.cpus().iter().map(|c| c.cpu_usage()).collect(),
        }
    }

    fn collect_memory(&self) -> MemoryInfo {
        MemoryInfo {
            used_bytes: self.system.used_memory(),
            total_bytes: self.system.total_memory(),
        }
    }

    fn collect_disks(&self) -> Vec<DiskInfo> {
        self.disks
            .iter()
            .map(|d| DiskInfo {
                name: d.name().to_string_lossy().into_owned(),
                used_bytes: d.total_space() - d.available_space(),
                total_bytes: d.total_space(),
            })
            .collect()
    }

    fn collect_networks(&self) -> Vec<NetworkInfo> {
        self.networks
            .iter()
            .map(|(name, data)| NetworkInfo {
                name: name.to_string(),
                received_bytes: data.total_received(),
                transmitted_bytes: data.total_transmitted(),
            })
            .collect()
    }

    fn collect_processes(&self) -> Vec<ProcessInfo> {
        let mut processes: Vec<_> = self
            .system
            .processes()
            .values()
            .map(|p| ProcessInfo {
                pid: p.pid().as_u32(),
                name: p.name().to_string_lossy().into_owned(),
                cpu_usage: p.cpu_usage(),
                memory_bytes: p.memory(),
            })
            .collect();

        processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());
        processes.truncate(15);
        processes
    }
}
