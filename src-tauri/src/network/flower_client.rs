use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowerConfig {
    pub server_address: String,
    pub server_port: u16,
    pub client_id: String,
    pub model_path: String,
    pub local_epochs: u32,
    pub batch_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingMetrics {
    pub round: u32,
    pub loss: f64,
    pub accuracy: Option<f64>,
    pub num_examples: u64,
    pub training_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientStatus {
    Disconnected,
    Connected,
    Training,
    Uploading,
    WaitingForRound,
}

pub struct FlowerClient {
    config: FlowerConfig,
    status: ClientStatus,
    current_round: u32,
    metrics_history: Vec<TrainingMetrics>,
}

impl FlowerClient {
    pub fn new(config: FlowerConfig) -> Self {
        Self {
            config,
            status: ClientStatus::Disconnected,
            current_round: 0,
            metrics_history: Vec::new(),
        }
    }
    
    pub fn connect(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        // TODO: Implement Flower protocol connection
        println!("Connecting to Flower server at {}:{}", 
                 self.config.server_address, self.config.server_port);
        
        self.status = ClientStatus::Connected;
        Ok(true)
    }
    
    pub fn disconnect(&mut self) {
        self.status = ClientStatus::Disconnected;
        println!("Disconnected from Flower server");
    }
    
    pub fn start_training_round(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !matches!(self.status, ClientStatus::Connected | ClientStatus::WaitingForRound) {
            return Err("Client not connected".into());
        }
        
        self.status = ClientStatus::Training;
        self.current_round += 1;
        
        // TODO: Implement actual federated training round
        println!("Starting training round {}", self.current_round);
        
        Ok(())
    }
    
    pub fn finish_training_round(&mut self, metrics: TrainingMetrics) -> Result<(), Box<dyn std::error::Error>> {
        self.metrics_history.push(metrics);
        self.status = ClientStatus::Uploading;
        
        // TODO: Upload model updates to server
        println!("Finished training round {}, uploading results", self.current_round);
        
        self.status = ClientStatus::WaitingForRound;
        Ok(())
    }
    
    pub fn get_status(&self) -> &ClientStatus {
        &self.status
    }
    
    pub fn get_current_round(&self) -> u32 {
        self.current_round
    }
    
    pub fn get_metrics_history(&self) -> &[TrainingMetrics] {
        &self.metrics_history
    }
    
    pub fn get_latest_metrics(&self) -> Option<&TrainingMetrics> {
        self.metrics_history.last()
    }
}
