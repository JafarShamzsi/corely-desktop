//! Core Application State Management System
//! 
//! This module provides comprehensive state management for the AI Training Marketplace
//! desktop application. It includes hardware profiling, setup status tracking,
//! authentication state, and real-time event system for frontend communication.

use std::sync::{Arc, Mutex, RwLock};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use thiserror::Error;
use tracing::{debug, info, warn, error, instrument};
use uuid::Uuid;

/// Custom error types for state management operations
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum StateError {
    #[error("Lock acquisition failed: {0}")]
    LockError(String),
    
    #[error("Invalid state transition from {from} to {to}")]
    InvalidTransition { from: String, to: String },
    
    #[error("Authentication error: {0}")]
    AuthError(String),
    
    #[error("Hardware detection failed: {0}")]
    HardwareError(String),
    
    #[error("Setup validation failed: {0}")]
    SetupError(String),
    
    #[error("Event system error: {0}")]
    EventError(String),
}

/// Setup status tracking with detailed progress information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SetupStatus {
    /// Initial system analysis and requirements check
    SystemAnalysis {
        progress: u8,        // 0-100
        details: String,
        started_at: DateTime<Utc>,
    },
    
    /// Hardware detection and capability assessment
    HardwareDetection {
        progress: u8,
        cpu_detected: bool,
        gpu_detected: bool,
        memory_detected: bool,
        started_at: DateTime<Utc>,
    },
    
    /// Driver compatibility and installation check
    DriverCheck {
        progress: u8,
        drivers_status: HashMap<String, bool>,
        recommendations: Vec<String>,
        started_at: DateTime<Utc>,
    },
    
    /// Environment setup and dependency installation
    EnvironmentSetup {
        progress: u8,
        dependencies_installed: Vec<String>,
        environment_ready: bool,
        started_at: DateTime<Utc>,
    },
    
    /// Setup completed successfully
    Complete {
        completed_at: DateTime<Utc>,
        summary: String,
    },
    
    /// Setup failed with error details
    Failed {
        error: String,
        failed_at: DateTime<Utc>,
        retry_count: u32,
    },
}

impl SetupStatus {
    /// Get the overall progress percentage
    pub fn progress(&self) -> u8 {
        match self {
            SetupStatus::SystemAnalysis { progress, .. } => *progress / 4,
            SetupStatus::HardwareDetection { progress, .. } => 25 + (*progress / 4),
            SetupStatus::DriverCheck { progress, .. } => 50 + (*progress / 4),
            SetupStatus::EnvironmentSetup { progress, .. } => 75 + (*progress / 4),
            SetupStatus::Complete { .. } => 100,
            SetupStatus::Failed { .. } => 0,
        }
    }
    
    /// Check if setup is in a completed state
    pub fn is_complete(&self) -> bool {
        matches!(self, SetupStatus::Complete { .. })
    }
    
    /// Check if setup has failed
    pub fn is_failed(&self) -> bool {
        matches!(self, SetupStatus::Failed { .. })
    }
}

/// Hardware profile information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareProfile {
    /// CPU information
    pub cpu: CpuInfo,
    /// GPU information (if available)
    pub gpu: Option<GpuInfo>,
    /// System memory information
    pub memory: MemoryInfo,
    /// Storage information
    pub storage: StorageInfo,
    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuInfo {
    pub name: String,
    pub cores: u32,
    pub threads: u32,
    pub base_frequency: f64,  // GHz
    pub max_frequency: f64,   // GHz
    pub cache_size: u64,      // bytes
    pub architecture: String,
    pub supports_avx: bool,
    pub supports_avx2: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    pub name: String,
    pub vendor: String,
    pub memory: u64,          // bytes
    pub compute_capability: Option<String>,
    pub cuda_cores: Option<u32>,
    pub driver_version: String,
    pub supports_ml: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
    pub total: u64,           // bytes
    pub available: u64,       // bytes
    pub speed: Option<u32>,   // MHz
    pub memory_type: String,  // DDR4, DDR5, etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageInfo {
    pub total_space: u64,     // bytes
    pub available_space: u64, // bytes
    pub storage_type: String, // SSD, HDD, NVMe
    pub read_speed: Option<f64>,  // MB/s
    pub write_speed: Option<f64>, // MB/s
}

/// Authentication state information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthState {
    pub is_authenticated: bool,
    pub user_id: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub token: Option<String>,
    pub token_expires_at: Option<DateTime<Utc>>,
    pub permissions: Vec<String>,
    pub last_login: Option<DateTime<Utc>>,
}

impl Default for AuthState {
    fn default() -> Self {
        Self {
            is_authenticated: false,
            user_id: None,
            username: None,
            email: None,
            token: None,
            token_expires_at: None,
            permissions: Vec::new(),
            last_login: None,
        }
    }
}

impl AuthState {
    /// Check if the current token is valid and not expired
    pub fn is_token_valid(&self) -> bool {
        if let (Some(_), Some(expires_at)) = (&self.token, &self.token_expires_at) {
            expires_at > &Utc::now()
        } else {
            false
        }
    }
    
    /// Check if user has a specific permission
    pub fn has_permission(&self, permission: &str) -> bool {
        self.is_authenticated && self.permissions.contains(&permission.to_string())
    }
}

/// Event types for real-time updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppEvent {
    /// Setup status changed
    SetupStatusChanged {
        old_status: SetupStatus,
        new_status: SetupStatus,
        timestamp: DateTime<Utc>,
    },
    
    /// Hardware profile updated
    HardwareProfileUpdated {
        profile: HardwareProfile,
        timestamp: DateTime<Utc>,
    },
    
    /// Authentication state changed
    AuthStateChanged {
        authenticated: bool,
        user_id: Option<String>,
        timestamp: DateTime<Utc>,
    },
    
    /// Training session started/stopped
    TrainingSessionChanged {
        session_id: Option<String>,
        model_name: Option<String>,
        status: String,
        timestamp: DateTime<Utc>,
    },
    
    /// System resource usage updated
    ResourceUsageUpdated {
        cpu_usage: f64,
        memory_usage: f64,
        gpu_usage: Option<f64>,
        timestamp: DateTime<Utc>,
    },
    
    /// Error occurred
    ErrorOccurred {
        error: StateError,
        context: String,
        timestamp: DateTime<Utc>,
    },
}

/// Event listener callback type
pub type EventListener = Box<dyn Fn(AppEvent) + Send + Sync>;

/// Main application state container
pub struct AppState {
    /// Hardware profile information
    pub hardware_profile: Arc<RwLock<Option<HardwareProfile>>>,
    
    /// Current setup status
    pub setup_status: Arc<RwLock<SetupStatus>>,
    
    /// Authentication state
    pub auth_state: Arc<RwLock<AuthState>>,
    
    /// Application configuration
    pub config: Arc<RwLock<AppConfig>>,
    
    /// Training session state
    pub training_state: Arc<RwLock<TrainingState>>,
    
    /// Event listeners for real-time updates
    event_listeners: Arc<Mutex<Vec<EventListener>>>,
    
    /// Event history for debugging
    event_history: Arc<Mutex<Vec<AppEvent>>>,
    
    /// Application startup timestamp
    pub started_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub user_id: Option<String>,
    pub api_endpoint: String,
    pub enable_distributed_training: bool,
    pub max_memory_usage_gb: u64,
    pub preferred_gpu: Option<String>,
    pub auto_save_interval: u64,      // seconds
    pub log_level: String,
    pub enable_telemetry: bool,
    pub data_directory: String,
    pub temp_directory: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            user_id: None,
            api_endpoint: "https://api.ai-training.com".to_string(),
            enable_distributed_training: false,
            max_memory_usage_gb: 8,
            preferred_gpu: None,
            auto_save_interval: 300,
            log_level: "info".to_string(),
            enable_telemetry: true,
            data_directory: "./data".to_string(),
            temp_directory: "./temp".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingState {
    pub is_training: bool,
    pub current_session_id: Option<String>,
    pub current_model: Option<String>,
    pub progress: f64,                // 0.0 - 1.0
    pub epoch: u32,
    pub batch: u32,
    pub loss: Option<f64>,
    pub accuracy: Option<f64>,
    pub training_start: Option<DateTime<Utc>>,
    pub estimated_completion: Option<DateTime<Utc>>,
}

impl Default for TrainingState {
    fn default() -> Self {
        Self {
            is_training: false,
            current_session_id: None,
            current_model: None,
            progress: 0.0,
            epoch: 0,
            batch: 0,
            loss: None,
            accuracy: None,
            training_start: None,
            estimated_completion: None,
        }
    }
}

impl AppState {
    /// Create a new application state instance
    #[instrument]
    pub fn new() -> Self {
        info!("Initializing application state");
        
        let initial_setup_status = SetupStatus::SystemAnalysis {
            progress: 0,
            details: "Starting system analysis...".to_string(),
            started_at: Utc::now(),
        };
        
        Self {
            hardware_profile: Arc::new(RwLock::new(None)),
            setup_status: Arc::new(RwLock::new(initial_setup_status)),
            auth_state: Arc::new(RwLock::new(AuthState::default())),
            config: Arc::new(RwLock::new(AppConfig::default())),
            training_state: Arc::new(RwLock::new(TrainingState::default())),
            event_listeners: Arc::new(Mutex::new(Vec::new())),
            event_history: Arc::new(Mutex::new(Vec::new())),
            started_at: Utc::now(),
        }
    }
    
    /// Update the setup status and emit event
    #[instrument(skip(self))]
    pub fn update_setup_status(&self, new_status: SetupStatus) -> Result<(), StateError> {
        let old_status = {
            let mut status = self.setup_status.write()
                .map_err(|e| StateError::LockError(format!("Failed to acquire write lock: {}", e)))?;
            
            let old = status.clone();
            *status = new_status.clone();
            old
        };
        
        info!("Setup status updated: {:?} -> {:?}", old_status, new_status);
        
        let event = AppEvent::SetupStatusChanged {
            old_status,
            new_status,
            timestamp: Utc::now(),
        };
        
        self.emit_event(event)?;
        Ok(())
    }
    
    /// Update hardware profile
    #[instrument(skip(self, profile))]
    pub fn update_hardware_profile(&self, profile: HardwareProfile) -> Result<(), StateError> {
        {
            let mut hw_profile = self.hardware_profile.write()
                .map_err(|e| StateError::LockError(format!("Failed to acquire write lock: {}", e)))?;
            *hw_profile = Some(profile.clone());
        }
        
        info!("Hardware profile updated");
        
        let event = AppEvent::HardwareProfileUpdated {
            profile,
            timestamp: Utc::now(),
        };
        
        self.emit_event(event)?;
        Ok(())
    }
    
    /// Update authentication state
    #[instrument(skip(self, auth_state))]
    pub fn update_auth_state(&self, auth_state: AuthState) -> Result<(), StateError> {
        let (authenticated, user_id) = {
            let mut state = self.auth_state.write()
                .map_err(|e| StateError::LockError(format!("Failed to acquire write lock: {}", e)))?;
            
            let authenticated = auth_state.is_authenticated;
            let user_id = auth_state.user_id.clone();
            
            *state = auth_state;
            (authenticated, user_id)
        };
        
        info!("Authentication state updated: authenticated={}", authenticated);
        
        let event = AppEvent::AuthStateChanged {
            authenticated,
            user_id,
            timestamp: Utc::now(),
        };
        
        self.emit_event(event)?;
        Ok(())
    }
    
    /// Start a training session
    #[instrument(skip(self))]
    pub fn start_training(&self, model_name: String) -> Result<String, StateError> {
        let session_id = Uuid::new_v4().to_string();
        
        {
            let mut training = self.training_state.write()
                .map_err(|e| StateError::LockError(format!("Failed to acquire write lock: {}", e)))?;
            
            training.is_training = true;
            training.current_session_id = Some(session_id.clone());
            training.current_model = Some(model_name.clone());
            training.progress = 0.0;
            training.epoch = 0;
            training.batch = 0;
            training.training_start = Some(Utc::now());
        }
        
        info!("Training session started: {} for model {}", session_id, model_name);
        
        let event = AppEvent::TrainingSessionChanged {
            session_id: Some(session_id.clone()),
            model_name: Some(model_name),
            status: "started".to_string(),
            timestamp: Utc::now(),
        };
        
        self.emit_event(event)?;
        Ok(session_id)
    }
    
    /// Stop the current training session
    #[instrument(skip(self))]
    pub fn stop_training(&self) -> Result<(), StateError> {
        let (session_id, model_name) = {
            let mut training = self.training_state.write()
                .map_err(|e| StateError::LockError(format!("Failed to acquire write lock: {}", e)))?;
            
            let session_id = training.current_session_id.clone();
            let model_name = training.current_model.clone();
            
            training.is_training = false;
            training.current_session_id = None;
            training.current_model = None;
            training.progress = 0.0;
            training.estimated_completion = None;
            
            (session_id, model_name)
        };
        
        info!("Training session stopped");
        
        let event = AppEvent::TrainingSessionChanged {
            session_id,
            model_name,
            status: "stopped".to_string(),
            timestamp: Utc::now(),
        };
        
        self.emit_event(event)?;
        Ok(())
    }
    
    /// Add an event listener
    #[instrument(skip(self, listener))]
    pub fn add_event_listener<F>(&self, listener: F) -> Result<(), StateError>
    where
        F: Fn(AppEvent) + Send + Sync + 'static,
    {
        let mut listeners = self.event_listeners.lock()
            .map_err(|e| StateError::LockError(format!("Failed to acquire lock: {}", e)))?;
        
        listeners.push(Box::new(listener));
        debug!("Event listener added, total listeners: {}", listeners.len());
        Ok(())
    }
    
    /// Emit an event to all listeners
    #[instrument(skip(self, event))]
    pub fn emit_event(&self, event: AppEvent) -> Result<(), StateError> {
        // Add to history
        {
            let mut history = self.event_history.lock()
                .map_err(|e| StateError::LockError(format!("Failed to acquire lock: {}", e)))?;
            history.push(event.clone());
            
            // Keep only last 1000 events
            if history.len() > 1000 {
                let target_len = history.len() - 1000;
                history.drain(0..target_len);
            }
        }
        
        // Notify listeners
        {
            let listeners = self.event_listeners.lock()
                .map_err(|e| StateError::LockError(format!("Failed to acquire lock: {}", e)))?;
            
            for listener in listeners.iter() {
                listener(event.clone());
            }
        }
        
        debug!("Event emitted: {:?}", event);
        Ok(())
    }
    
    /// Get current setup status
    pub fn get_setup_status(&self) -> Result<SetupStatus, StateError> {
        let status = self.setup_status.read()
            .map_err(|e| StateError::LockError(format!("Failed to acquire read lock: {}", e)))?;
        Ok(status.clone())
    }
    
    /// Get current hardware profile
    pub fn get_hardware_profile(&self) -> Result<Option<HardwareProfile>, StateError> {
        let profile = self.hardware_profile.read()
            .map_err(|e| StateError::LockError(format!("Failed to acquire read lock: {}", e)))?;
        Ok(profile.clone())
    }
    
    /// Get current authentication state
    pub fn get_auth_state(&self) -> Result<AuthState, StateError> {
        let state = self.auth_state.read()
            .map_err(|e| StateError::LockError(format!("Failed to acquire read lock: {}", e)))?;
        Ok(state.clone())
    }
    
    /// Get current training state
    pub fn get_training_state(&self) -> Result<TrainingState, StateError> {
        let state = self.training_state.read()
            .map_err(|e| StateError::LockError(format!("Failed to acquire read lock: {}", e)))?;
        Ok(state.clone())
    }
    
    /// Get application configuration
    pub fn get_config(&self) -> Result<AppConfig, StateError> {
        let config = self.config.read()
            .map_err(|e| StateError::LockError(format!("Failed to acquire read lock: {}", e)))?;
        Ok(config.clone())
    }
    
    /// Update application configuration
    #[instrument(skip(self, config))]
    pub fn update_config(&self, config: AppConfig) -> Result<(), StateError> {
        {
            let mut app_config = self.config.write()
                .map_err(|e| StateError::LockError(format!("Failed to acquire write lock: {}", e)))?;
            *app_config = config;
        }
        
        info!("Application configuration updated");
        Ok(())
    }
    
    /// Get recent event history
    pub fn get_event_history(&self, limit: usize) -> Result<Vec<AppEvent>, StateError> {
        let history = self.event_history.lock()
            .map_err(|e| StateError::LockError(format!("Failed to acquire lock: {}", e)))?;
        
        let start_idx = if history.len() > limit {
            history.len() - limit
        } else {
            0
        };
        
        Ok(history[start_idx..].to_vec())
    }
    
    /// Check if application is ready for use
    pub fn is_ready(&self) -> Result<bool, StateError> {
        let setup_status = self.get_setup_status()?;
        let hardware_profile = self.get_hardware_profile()?;
        
        Ok(setup_status.is_complete() && hardware_profile.is_some())
    }
    
    /// Get application uptime in seconds
    pub fn uptime_seconds(&self) -> i64 {
        (Utc::now() - self.started_at).num_seconds()
    }
}
