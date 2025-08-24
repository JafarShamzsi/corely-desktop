use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareInfo {
    pub cpu_model: String,
    pub cpu_cores: usize,
    pub memory_total: u64,
    pub memory_available: u64,
    pub gpus: Vec<GpuInfo>,
    pub storage_devices: Vec<StorageInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    pub name: String,
    pub memory_mb: Option<u64>,
    pub vendor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageInfo {
    pub name: String,
    pub total_space: u64,
    pub available_space: u64,
    pub mount_point: String,
}

pub fn detect_hardware() -> Result<HardwareInfo, String> {
    use sysinfo::System;
    
    let mut sys = System::new_all();
    sys.refresh_all();
    
    // Basic CPU information
    let cpu_model = sys.cpus().first()
        .map(|cpu| cpu.brand().to_string())
        .unwrap_or_else(|| "Unknown CPU".to_string());
    
    // TODO: Implement GPU detection using platform-specific APIs
    let gpus = vec![];
    
    // Storage information - temporarily empty until we fix the API
    let storage_devices = vec![];
    
    Ok(HardwareInfo {
        cpu_model,
        cpu_cores: sys.cpus().len(),
        memory_total: sys.total_memory(),
        memory_available: sys.available_memory(),
        gpus,
        storage_devices,
    })
}
