use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkNode {
    pub id: String,
    pub address: String,
    pub port: u16,
    pub public_key: Option<String>,
    pub capabilities: Vec<String>,
    pub last_seen: String,
    pub status: NodeStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeStatus {
    Online,
    Offline,
    Busy,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingJob {
    pub id: String,
    pub name: String,
    pub model_type: String,
    pub participants: Vec<String>,
    pub coordinator: String,
    pub status: JobStatus,
    pub created_at: String,
    pub estimated_completion: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

pub struct DistributedTrainingManager {
    nodes: HashMap<String, NetworkNode>,
    active_jobs: HashMap<String, TrainingJob>,
    local_node_id: String,
}

impl DistributedTrainingManager {
    pub fn new(local_node_id: String) -> Self {
        Self {
            nodes: HashMap::new(),
            active_jobs: HashMap::new(),
            local_node_id,
        }
    }
    
    pub fn discover_nodes(&mut self) -> Result<Vec<NetworkNode>, Box<dyn std::error::Error>> {
        // TODO: Implement network discovery (mDNS, broadcast, etc.)
        Ok(vec![])
    }
    
    pub fn register_node(&mut self, node: NetworkNode) {
        self.nodes.insert(node.id.clone(), node);
    }
    
    pub fn create_training_job(&mut self, job: TrainingJob) -> Result<String, Box<dyn std::error::Error>> {
        let job_id = job.id.clone();
        self.active_jobs.insert(job_id.clone(), job);
        Ok(job_id)
    }
    
    pub fn join_training_job(&mut self, job_id: &str) -> Result<bool, Box<dyn std::error::Error>> {
        // TODO: Implement joining logic
        Ok(true)
    }
    
    pub fn get_active_nodes(&self) -> Vec<&NetworkNode> {
        self.nodes.values()
            .filter(|node| matches!(node.status, NodeStatus::Online))
            .collect()
    }
    
    pub fn get_training_jobs(&self) -> Vec<&TrainingJob> {
        self.active_jobs.values().collect()
    }
}
