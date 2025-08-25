//! State Management Commands
//! 
//! Tauri commands for interacting with the application state from the frontend.

use tauri::{command, State};
use tracing::{instrument, info, error};
use crate::core::{
    AppState, AppConfig, SetupStatus, HardwareProfile, 
    AuthState, TrainingState, StateError, AppEvent
};

/// Get the current setup status
#[command]
#[instrument(skip(state))]
pub async fn get_setup_status(state: State<'_, AppState>) -> Result<SetupStatus, String> {
    info!("Getting setup status");
    
    state.get_setup_status()
        .map_err(|e| {
            error!("Failed to get setup status: {}", e);
            e.to_string()
        })
}

/// Get the current hardware profile
#[command]
#[instrument(skip(state))]
pub async fn get_hardware_profile(state: State<'_, AppState>) -> Result<Option<HardwareProfile>, String> {
    info!("Getting hardware profile");
    
    state.get_hardware_profile()
        .map_err(|e| {
            error!("Failed to get hardware profile: {}", e);
            e.to_string()
        })
}

/// Get the current authentication state
#[command]
#[instrument(skip(state))]
pub async fn get_auth_state(state: State<'_, AppState>) -> Result<AuthState, String> {
    info!("Getting authentication state");
    
    state.get_auth_state()
        .map_err(|e| {
            error!("Failed to get auth state: {}", e);
            e.to_string()
        })
}

/// Get the current training state
#[command]
#[instrument(skip(state))]
pub async fn get_training_state(state: State<'_, AppState>) -> Result<TrainingState, String> {
    info!("Getting training state");
    
    state.get_training_state()
        .map_err(|e| {
            error!("Failed to get training state: {}", e);
            e.to_string()
        })
}

/// Get the application configuration
#[command]
#[instrument(skip(state))]
pub async fn get_app_config(state: State<'_, AppState>) -> Result<AppConfig, String> {
    info!("Getting application config");
    
    state.get_config()
        .map_err(|e| {
            error!("Failed to get app config: {}", e);
            e.to_string()
        })
}

/// Update the application configuration
#[command]
#[instrument(skip(state, config))]
pub async fn update_app_config(
    state: State<'_, AppState>,
    config: AppConfig,
) -> Result<(), String> {
    info!("Updating application config");
    
    state.update_config(config)
        .map_err(|e| {
            error!("Failed to update app config: {}", e);
            e.to_string()
        })
}

/// Start a training session
#[command]
#[instrument(skip(state))]
pub async fn start_training_session(
    state: State<'_, AppState>,
    model_name: String,
) -> Result<String, String> {
    info!("Starting training session for model: {}", model_name);
    
    state.start_training(model_name)
        .map_err(|e| {
            error!("Failed to start training session: {}", e);
            e.to_string()
        })
}

/// Stop the current training session
#[command]
#[instrument(skip(state))]
pub async fn stop_training_session(state: State<'_, AppState>) -> Result<(), String> {
    info!("Stopping training session");
    
    state.stop_training()
        .map_err(|e| {
            error!("Failed to stop training session: {}", e);
            e.to_string()
        })
}

/// Check if the application is ready for use
#[command]
#[instrument(skip(state))]
pub async fn is_app_ready(state: State<'_, AppState>) -> Result<bool, String> {
    info!("Checking if app is ready");
    
    state.is_ready()
        .map_err(|e| {
            error!("Failed to check app readiness: {}", e);
            e.to_string()
        })
}

/// Get application uptime in seconds
#[command]
#[instrument(skip(state))]
pub async fn get_app_uptime(state: State<'_, AppState>) -> Result<i64, String> {
    info!("Getting application uptime");
    
    Ok(state.uptime_seconds())
}

/// Get recent event history
#[command]
#[instrument(skip(state))]
pub async fn get_event_history(
    state: State<'_, AppState>,
    limit: Option<usize>,
) -> Result<Vec<AppEvent>, String> {
    let limit = limit.unwrap_or(50);
    info!("Getting event history (limit: {})", limit);
    
    state.get_event_history(limit)
        .map_err(|e| {
            error!("Failed to get event history: {}", e);
            e.to_string()
        })
}

/// Authenticate user with credentials
#[command]
#[instrument(skip(state, _password))]
pub async fn authenticate_user(
    state: State<'_, AppState>,
    username: String,
    _password: String,
) -> Result<AuthState, String> {
    info!("Authenticating user: {}", username);
    
    // This is a placeholder implementation
    // In a real application, you would validate credentials against a backend
    let auth_state = AuthState {
        is_authenticated: true,
        user_id: Some(uuid::Uuid::new_v4().to_string()),
        username: Some(username),
        email: Some("user@example.com".to_string()),
        token: Some("dummy_token".to_string()),
        token_expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(24)),
        permissions: vec!["read".to_string(), "write".to_string()],
        last_login: Some(chrono::Utc::now()),
    };
    
    state.update_auth_state(auth_state.clone())
        .map_err(|e| {
            error!("Failed to update auth state: {}", e);
            e.to_string()
        })?;
    
    Ok(auth_state)
}

/// Logout the current user
#[command]
#[instrument(skip(state))]
pub async fn logout_user(state: State<'_, AppState>) -> Result<(), String> {
    info!("Logging out user");
    
    let auth_state = AuthState::default();
    
    state.update_auth_state(auth_state)
        .map_err(|e| {
            error!("Failed to logout user: {}", e);
            e.to_string()
        })
}
