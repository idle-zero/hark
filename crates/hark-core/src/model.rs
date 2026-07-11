#[derive(Debug, Clone, Default)]
pub struct SystemSnapshot {
    pub cpu: CpuInfo,
    pub memory: MemoryInfo,
    pub disks: Vec<DiskInfo>,
    pub networks: Vec<NetworkInfo>,
    pub processes: Vec<ProcessInfo>,
}

#[derive(Debug, Clone, Default)]
pub struct CpuInfo {
    pub global_usage: f32,
    pub cores: Vec<f32>,
}

#[derive(Debug, Clone, Default)]
pub struct MemoryInfo {
    pub used_bytes: u64,
    pub total_bytes: u64,
}

#[derive(Debug, Clone, Default)]
pub struct DiskInfo {
    pub name: String,
    pub used_bytes: u64,
    pub total_bytes: u64,
}

#[derive(Debug, Clone, Default)]
pub struct NetworkInfo {
    pub name: String,
    pub received_bytes: u64,
    pub transmitted_bytes: u64,
}

#[derive(Debug, Clone, Default)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_bytes: u64,
}
