use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    pub allowed_directories: Vec<PathBuf>,
    pub allowed_network_hosts: Vec<String>,
    pub max_memory_mb: u64,
    pub max_cpu_percent: u32,
    pub allow_gpu_access: bool,
    pub timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxedProcess {
    pub id: String,
    pub name: String,
    pub config: SandboxConfig,
    pub status: ProcessStatus,
    pub created_at: String,
    pub memory_usage_mb: u64,
    pub cpu_usage_percent: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessStatus {
    Starting,
    Running,
    Stopped,
    Failed,
    TimedOut,
}

pub struct SandboxManager {
    processes: HashMap<String, SandboxedProcess>,
    default_config: SandboxConfig,
}

impl SandboxManager {
    pub fn new() -> Self {
        let default_config = SandboxConfig {
            allowed_directories: vec![
                dirs::data_dir().unwrap_or_default().join("ai-training-desktop"),
                std::env::temp_dir(),
            ],
            allowed_network_hosts: vec!["127.0.0.1".to_string()],
            max_memory_mb: 4096, // 4GB default
            max_cpu_percent: 80,
            allow_gpu_access: false,
            timeout_seconds: 3600, // 1 hour default
        };
        
        Self {
            processes: HashMap::new(),
            default_config,
        }
    }
    
    pub fn create_sandbox(&mut self, name: String, config: Option<SandboxConfig>) -> Result<String, Box<dyn std::error::Error>> {
        let config = config.unwrap_or_else(|| self.default_config.clone());
        let process_id = uuid::Uuid::new_v4().to_string();
        
        let sandboxed_process = SandboxedProcess {
            id: process_id.clone(),
            name,
            config,
            status: ProcessStatus::Starting,
            created_at: chrono::Utc::now().to_rfc3339(),
            memory_usage_mb: 0,
            cpu_usage_percent: 0.0,
        };
        
        self.processes.insert(process_id.clone(), sandboxed_process);
        
        // TODO: Implement actual process sandboxing
        // This would involve:
        // - Setting up filesystem isolation (chroot, bind mounts)
        // - Network restrictions (iptables, network namespaces)
        // - Resource limits (cgroups, ulimits)
        // - Security contexts (AppArmor, SELinux)
        
        Ok(process_id)
    }
    
    pub fn start_process(&mut self, process_id: &str, command: &str, args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let process = self.processes.get_mut(process_id)
            .ok_or("Process not found")?;
        
        // TODO: Start the actual sandboxed process
        println!("Starting sandboxed process: {} {}", command, args.join(" "));
        
        process.status = ProcessStatus::Running;
        Ok(())
    }
    
    pub fn stop_process(&mut self, process_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let process = self.processes.get_mut(process_id)
            .ok_or("Process not found")?;
        
        // TODO: Stop the actual process
        process.status = ProcessStatus::Stopped;
        Ok(())
    }
    
    pub fn get_process_status(&self, process_id: &str) -> Option<&SandboxedProcess> {
        self.processes.get(process_id)
    }
    
    pub fn list_processes(&self) -> Vec<&SandboxedProcess> {
        self.processes.values().collect()
    }
    
    pub fn cleanup_stopped_processes(&mut self) {
        self.processes.retain(|_, process| {
            !matches!(process.status, ProcessStatus::Stopped | ProcessStatus::Failed | ProcessStatus::TimedOut)
        });
    }
    
    pub fn update_resource_usage(&mut self, process_id: &str, memory_mb: u64, cpu_percent: f32) -> Result<(), Box<dyn std::error::Error>> {
        let process = self.processes.get_mut(process_id)
            .ok_or("Process not found")?;
        
        process.memory_usage_mb = memory_mb;
        process.cpu_usage_percent = cpu_percent;
        
        // Check limits
        if memory_mb > process.config.max_memory_mb {
            println!("Process {} exceeded memory limit", process_id);
            process.status = ProcessStatus::Failed;
        }
        
        if cpu_percent > process.config.max_cpu_percent as f32 {
            println!("Process {} exceeded CPU limit", process_id);
        }
        
        Ok(())
    }
}
