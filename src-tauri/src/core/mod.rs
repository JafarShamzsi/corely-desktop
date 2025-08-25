//! Core application modules
//! 
//! This module contains the fundamental components for application state management,
//! configuration handling, and core business logic.

pub mod app_state;
pub mod config;

#[cfg(test)]
pub mod examples;

// Re-export main types for easier access
pub use app_state::{
    AppState, AppConfig, AppEvent, StateError, SetupStatus,
    HardwareProfile, AuthState, TrainingState
};
