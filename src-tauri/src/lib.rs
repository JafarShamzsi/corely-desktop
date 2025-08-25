//! AI Training Marketplace Desktop Application
//! 
//! Core library for the Tauri-based desktop application that provides
//! AI training capabilities, model management, and distributed training.

// Modules
mod commands;
mod core;
mod system;
mod ml;
mod network;
mod security;
mod utils;

use std::sync::Arc;

use commands::{
    greet::greet,
    system_commands::{get_system_info},
    ml_commands::{list_available_models, load_model},
    network_commands::{discover_network_nodes, connect_to_flower_server},
    state_commands::{
        get_setup_status, get_hardware_profile, get_auth_state, get_training_state,
        get_app_config, update_app_config, start_training_session, stop_training_session,
        is_app_ready, get_app_uptime, get_event_history, authenticate_user, logout_user
    },
    auth_commands::{
        init_auth_manager, register_user, login_user, oauth2_login, get_oauth2_auth_url,
        validate_token, refresh_access_token, secure_logout_user, 
        get_current_user, add_device_fingerprint, get_user_sessions, revoke_all_sessions,
        generate_device_fingerprint, cleanup_expired_auth
    },
};

use core::AppState;
use security::auth::AuthManager;
use tracing::{info, error};
use tracing_subscriber;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Load environment variables from .env file (if it exists)
    // This allows for easy configuration in development
    if let Err(e) = dotenvy::dotenv() {
        // Don't panic if .env file doesn't exist - it's optional
        eprintln!("Note: .env file not found or couldn't be loaded: {}", e);
        eprintln!("Environment variables will be loaded from system environment only.");
    }

    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .init();

    info!("Starting AI Training Marketplace application");

    // Initialize application state
    let app_state = AppState::new();

    // Initialize secure authentication manager
    let auth_manager = match AuthManager::new(
        std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-super-secret-jwt-key-change-in-production".to_string()),
        std::env::var("GOOGLE_CLIENT_ID").ok(),
        std::env::var("GOOGLE_CLIENT_SECRET").ok(),
        std::env::var("GITHUB_CLIENT_ID").ok(),
        std::env::var("GITHUB_CLIENT_SECRET").ok(),
        std::env::var("OAUTH_REDIRECT_URL").unwrap_or_else(|_| "http://localhost:1420/auth/callback".to_string()),
        "ai-training-desktop".to_string(),
        "ai-training-desktop-users".to_string(),
    ) {
        Ok(manager) => Arc::new(manager),
        Err(e) => {
            error!("Failed to initialize authentication manager: {}", e);
            panic!("Authentication manager initialization failed");
        }
    };

    info!("Authentication manager initialized successfully");

    // Set up event listener for debugging
    if let Err(e) = app_state.add_event_listener(|event| {
        info!("Event emitted: {:?}", event);
    }) {
        error!("Failed to add event listener: {}", e);
    }

    info!("Application state initialized successfully");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .manage(auth_manager)
        .invoke_handler(tauri::generate_handler![
            // Basic commands
            greet,
            // System commands
            get_system_info,
            // ML commands
            list_available_models,
            load_model,
            // Network commands
            discover_network_nodes,
            connect_to_flower_server,
            // State management commands
            get_setup_status,
            get_hardware_profile,
            get_auth_state,
            get_training_state,
            get_app_config,
            update_app_config,
            start_training_session,
            stop_training_session,
            is_app_ready,
            get_app_uptime,
            get_event_history,
            authenticate_user,
            logout_user,
            // Secure authentication commands
            init_auth_manager,
            register_user,
            login_user,
            oauth2_login,
            get_oauth2_auth_url,
            validate_token,
            refresh_access_token,
            secure_logout_user,
            get_current_user,
            add_device_fingerprint,
            get_user_sessions,
            revoke_all_sessions,
            generate_device_fingerprint,
            cleanup_expired_auth
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
