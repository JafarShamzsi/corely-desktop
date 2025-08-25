//! Core Application State Management System - Usage Examples
//! 
//! This file demonstrates how to use the comprehensive state management system
//! for the AI Training Marketplace desktop application.

use std::sync::Arc;
use chrono::Utc;
use crate::core::{
    AppState, SetupStatus, HardwareProfile, AuthState, AppEvent
};
use crate::core::app_state::{CpuInfo, GpuInfo, MemoryInfo, StorageInfo};

/// Example usage of the AppState system
pub async fn demonstrate_state_management() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 AI Training Marketplace State Management Demo");
    
    // Initialize application state
    let state = Arc::new(AppState::new());
    println!("✅ Application state initialized");
    
    // Add an event listener to monitor state changes
    state.add_event_listener(|event| {
        match event {
            AppEvent::SetupStatusChanged { old_status, new_status, timestamp } => {
                println!("📈 Setup Status: {:?} -> {:?} at {}", 
                    old_status, new_status, timestamp);
            }
            AppEvent::HardwareProfileUpdated { profile, timestamp } => {
                println!("💻 Hardware Profile Updated at {}: CPU={}, GPU={:?}", 
                    timestamp, profile.cpu.name, 
                    profile.gpu.as_ref().map(|g| &g.name));
            }
            AppEvent::AuthStateChanged { authenticated, user_id, timestamp } => {
                println!("🔐 Auth State Changed at {}: authenticated={}, user={:?}", 
                    timestamp, authenticated, user_id);
            }
            AppEvent::TrainingSessionChanged { session_id, model_name, status, timestamp } => {
                println!("🤖 Training Session at {}: {} for model {:?} ({})", 
                    timestamp, status, model_name, 
                    session_id.as_deref().unwrap_or("unknown"));
            }
            _ => println!("📊 Event: {:?}", event),
        }
    })?;
    
    // Simulate setup process
    println!("\n🔧 Starting setup process...");
    
    // Step 1: System Analysis
    state.update_setup_status(SetupStatus::SystemAnalysis {
        progress: 25,
        details: "Analyzing system requirements...".to_string(),
        started_at: Utc::now(),
    })?;
    
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    // Step 2: Hardware Detection
    state.update_setup_status(SetupStatus::HardwareDetection {
        progress: 50,
        cpu_detected: true,
        gpu_detected: true,
        memory_detected: true,
        started_at: Utc::now(),
    })?;
    
    // Update hardware profile
    let hardware_profile = HardwareProfile {
        cpu: CpuInfo {
            name: "Intel Core i7-12700K".to_string(),
            cores: 12,
            threads: 20,
            base_frequency: 3.6,
            max_frequency: 5.0,
            cache_size: 25 * 1024 * 1024, // 25MB
            architecture: "x86_64".to_string(),
            supports_avx: true,
            supports_avx2: true,
        },
        gpu: Some(GpuInfo {
            name: "NVIDIA RTX 4070".to_string(),
            vendor: "NVIDIA".to_string(),
            memory: 12 * 1024 * 1024 * 1024, // 12GB
            compute_capability: Some("8.9".to_string()),
            cuda_cores: Some(5888),
            driver_version: "546.01".to_string(),
            supports_ml: true,
        }),
        memory: MemoryInfo {
            total: 32 * 1024 * 1024 * 1024, // 32GB
            available: 24 * 1024 * 1024 * 1024, // 24GB
            speed: Some(3200),
            memory_type: "DDR4".to_string(),
        },
        storage: StorageInfo {
            total_space: 1024 * 1024 * 1024 * 1024, // 1TB
            available_space: 512 * 1024 * 1024 * 1024, // 512GB
            storage_type: "NVMe SSD".to_string(),
            read_speed: Some(3500.0),
            write_speed: Some(3000.0),
        },
        last_updated: Utc::now(),
    };
    
    state.update_hardware_profile(hardware_profile)?;
    
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    // Step 3: Driver Check
    let mut drivers_status = std::collections::HashMap::new();
    drivers_status.insert("NVIDIA Driver".to_string(), true);
    drivers_status.insert("CUDA Toolkit".to_string(), true);
    drivers_status.insert("PyTorch".to_string(), false);
    
    state.update_setup_status(SetupStatus::DriverCheck {
        progress: 75,
        drivers_status,
        recommendations: vec![
            "Install PyTorch for GPU acceleration".to_string(),
            "Update NVIDIA driver to latest version".to_string(),
        ],
        started_at: Utc::now(),
    })?;
    
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    // Step 4: Environment Setup
    state.update_setup_status(SetupStatus::EnvironmentSetup {
        progress: 90,
        dependencies_installed: vec![
            "Python 3.11".to_string(),
            "Rust 1.75".to_string(),
            "Node.js 20".to_string(),
        ],
        environment_ready: true,
        started_at: Utc::now(),
    })?;
    
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    // Step 5: Complete
    state.update_setup_status(SetupStatus::Complete {
        completed_at: Utc::now(),
        summary: "System ready for AI training workloads".to_string(),
    })?;
    
    // Simulate authentication
    println!("\n🔐 Testing authentication...");
    let auth_state = AuthState {
        is_authenticated: true,
        user_id: Some("user_123".to_string()),
        username: Some("ai_researcher".to_string()),
        email: Some("researcher@university.edu".to_string()),
        token: Some("jwt_token_here".to_string()),
        token_expires_at: Some(Utc::now() + chrono::Duration::hours(24)),
        permissions: vec!["read".to_string(), "write".to_string(), "train".to_string()],
        last_login: Some(Utc::now()),
    };
    
    state.update_auth_state(auth_state)?;
    
    // Simulate training session
    println!("\n🤖 Starting training session...");
    let session_id = state.start_training("resnet50_custom".to_string())?;
    println!("Training session started with ID: {}", session_id);
    
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    
    // Stop training
    state.stop_training()?;
    println!("Training session stopped");
    
    // Display current state
    println!("\n📊 Current Application State:");
    println!("- Setup Status: {:?}", state.get_setup_status()?);
    println!("- Hardware Profile: Available");
    println!("- Auth State: {:?}", state.get_auth_state()?.is_authenticated);
    println!("- Training State: {:?}", state.get_training_state()?.is_training);
    println!("- App Ready: {}", state.is_ready()?);
    println!("- Uptime: {} seconds", state.uptime_seconds());
    
    // Show recent events
    println!("\n📝 Recent Events:");
    let events = state.get_event_history(5)?;
    for (i, event) in events.iter().enumerate() {
        println!("  {}. {:?}", i + 1, event);
    }
    
    println!("\n✅ State management demonstration complete!");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{AppState, SetupStatus};
    
    #[tokio::test]
    async fn test_basic_state_operations() {
        let state = AppState::new();
        
        // Test initial state
        assert!(state.get_setup_status().is_ok());
        assert!(state.get_hardware_profile().unwrap().is_none());
        assert!(!state.get_auth_state().unwrap().is_authenticated);
        assert!(!state.get_training_state().unwrap().is_training);
        
        // Test setup status update
        let new_status = SetupStatus::SystemAnalysis {
            progress: 50,
            details: "Test analysis".to_string(),
            started_at: Utc::now(),
        };
        
        assert!(state.update_setup_status(new_status).is_ok());
        assert_eq!(state.get_setup_status().unwrap().progress(), 12); // 50/4 = 12.5 -> 12
    }
    
    #[tokio::test]
    async fn test_training_session_lifecycle() {
        let state = AppState::new();
        
        // Start training
        let session_id = state.start_training("test_model".to_string()).unwrap();
        assert!(!session_id.is_empty());
        
        let training_state = state.get_training_state().unwrap();
        assert!(training_state.is_training);
        assert_eq!(training_state.current_model, Some("test_model".to_string()));
        assert_eq!(training_state.current_session_id, Some(session_id));
        
        // Stop training
        assert!(state.stop_training().is_ok());
        
        let training_state = state.get_training_state().unwrap();
        assert!(!training_state.is_training);
        assert_eq!(training_state.current_model, None);
        assert_eq!(training_state.current_session_id, None);
    }
    
    #[tokio::test]
    async fn test_event_system() {
        let state = AppState::new();
        let events_received = Arc::new(std::sync::Mutex::new(Vec::new()));
        let events_clone = events_received.clone();
        
        // Add event listener
        state.add_event_listener(move |event| {
            let mut events = events_clone.lock().unwrap();
            events.push(event);
        }).unwrap();
        
        // Trigger some events
        let auth_state = AuthState {
            is_authenticated: true,
            user_id: Some("test_user".to_string()),
            ..Default::default()
        };
        
        state.update_auth_state(auth_state).unwrap();
        
        // Give event system time to process
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        
        let events = events_received.lock().unwrap();
        assert!(!events.is_empty());
    }
}
