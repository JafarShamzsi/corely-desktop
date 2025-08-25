//! Authentication Commands
//! 
//! Tauri commands for secure authentication operations from the frontend.

use tauri::{command, State};
use tracing::{instrument, info, error, warn};
use std::sync::Arc;

use crate::core::AppState;
use crate::security::auth::{
    AuthManager, AuthError, LoginRequest, RegisterRequest, OAuth2LoginRequest, 
    OAuth2Provider, AuthResponse, OAuth2AuthUrl, User
};

/// Initialize OAuth2 authentication manager
#[command]
#[instrument(skip(auth_manager, app_state))]
pub async fn init_auth_manager(
    auth_manager: State<'_, Arc<AuthManager>>,
    app_state: State<'_, AppState>,
) -> Result<bool, String> {
    info!("Initializing authentication manager");
    
    // Perform any initialization tasks
    // For now, just return success
    Ok(true)
}

/// Register a new user
#[command]
#[instrument(skip(auth_manager, app_state, request), fields(username = %request.username, email = %request.email))]
pub async fn register_user(
    auth_manager: State<'_, Arc<AuthManager>>,
    app_state: State<'_, AppState>,
    request: RegisterRequest,
) -> Result<AuthResponse, String> {
    info!("Registering new user: {}", request.username);
    
    let response = auth_manager.register_user(request).await
        .map_err(|e| {
            error!("User registration failed: {}", e);
            e.to_string()
        })?;

    // Update app state
    auth_manager.update_app_state_auth(&app_state, &response.user, &response.session_id).await
        .map_err(|e| {
            error!("Failed to update app state after registration: {}", e);
            e.to_string()
        })?;

    info!("User registered successfully: {}", response.user.email);
    Ok(response)
}

/// Authenticate user with username/password
#[command]
#[instrument(skip(auth_manager, app_state, request), fields(username = %request.username))]
pub async fn login_user(
    auth_manager: State<'_, Arc<AuthManager>>,
    app_state: State<'_, AppState>,
    request: LoginRequest,
) -> Result<AuthResponse, String> {
    info!("User login attempt: {}", request.username);
    
    let response = auth_manager.login(request).await
        .map_err(|e| {
            error!("User login failed: {}", e);
            match e {
                AuthError::InvalidCredentials => "Invalid username or password".to_string(),
                AuthError::PermissionDenied => "Account is disabled".to_string(),
                AuthError::DeviceFingerprintMismatch => "Device not recognized".to_string(),
                _ => "Authentication failed".to_string(),
            }
        })?;

    // Update app state
    auth_manager.update_app_state_auth(&app_state, &response.user, &response.session_id).await
        .map_err(|e| {
            error!("Failed to update app state after login: {}", e);
            e.to_string()
        })?;

    info!("User logged in successfully: {}", response.user.email);
    Ok(response)
}

/// OAuth2 login
#[command]
#[instrument(skip(auth_manager, app_state, request))]
pub async fn oauth2_login(
    auth_manager: State<'_, Arc<AuthManager>>,
    app_state: State<'_, AppState>,
    request: OAuth2LoginRequest,
) -> Result<AuthResponse, String> {
    info!("OAuth2 login attempt for provider: {:?}", request.provider);
    
    let response = auth_manager.oauth2_login(request).await
        .map_err(|e| {
            error!("OAuth2 login failed: {}", e);
            e.to_string()
        })?;

    // Update app state
    auth_manager.update_app_state_auth(&app_state, &response.user, &response.session_id).await
        .map_err(|e| {
            error!("Failed to update app state after OAuth2 login: {}", e);
            e.to_string()
        })?;

    info!("OAuth2 login successful: {}", response.user.email);
    Ok(response)
}

/// Get OAuth2 authorization URL
#[command]
#[instrument(skip(auth_manager))]
pub async fn get_oauth2_auth_url(
    auth_manager: State<'_, Arc<AuthManager>>,
    provider: OAuth2Provider,
) -> Result<OAuth2AuthUrl, String> {
    info!("Getting OAuth2 auth URL for provider: {:?}", provider);
    
    auth_manager.get_oauth2_auth_url(provider)
        .map_err(|e| {
            error!("Failed to get OAuth2 auth URL: {}", e);
            e.to_string()
        })
}

/// Validate JWT token
#[command]
#[instrument(skip(auth_manager, token))]
pub async fn validate_token(
    auth_manager: State<'_, Arc<AuthManager>>,
    token: String,
) -> Result<bool, String> {
    match auth_manager.validate_token(&token).await {
        Ok(_) => {
            info!("Token validation successful");
            Ok(true)
        }
        Err(e) => {
            warn!("Token validation failed: {}", e);
            Ok(false)
        }
    }
}

/// Refresh access token
#[command]
#[instrument(skip(auth_manager, app_state, refresh_token))]
pub async fn refresh_access_token(
    auth_manager: State<'_, Arc<AuthManager>>,
    app_state: State<'_, AppState>,
    refresh_token: String,
) -> Result<AuthResponse, String> {
    info!("Refreshing access token");
    
    let response = auth_manager.refresh_token(&refresh_token).await
        .map_err(|e| {
            error!("Token refresh failed: {}", e);
            match e {
                AuthError::TokenExpired => "Refresh token expired".to_string(),
                AuthError::TokenInvalid(_) => "Invalid refresh token".to_string(),
                _ => "Token refresh failed".to_string(),
            }
        })?;

    // Update app state
    auth_manager.update_app_state_auth(&app_state, &response.user, &response.session_id).await
        .map_err(|e| {
            error!("Failed to update app state after token refresh: {}", e);
            e.to_string()
        })?;

    info!("Access token refreshed successfully");
    Ok(response)
}

/// Logout user
#[command]
#[instrument(skip(auth_manager, app_state, token))]
pub async fn secure_logout_user(
    auth_manager: State<'_, Arc<AuthManager>>,
    app_state: State<'_, AppState>,
    token: String,
) -> Result<bool, String> {
    info!("User logout");
    
    auth_manager.logout(&token).await
        .map_err(|e| {
            error!("Logout failed: {}", e);
            e.to_string()
        })?;

    // Clear app state
    auth_manager.clear_app_state_auth(&app_state).await
        .map_err(|e| {
            error!("Failed to clear app state after logout: {}", e);
            e.to_string()
        })?;

    info!("User logged out successfully");
    Ok(true)
}

/// Get current user information
#[command]
pub async fn get_current_user(
    auth_manager: State<'_, Arc<AuthManager>>,
    token: String,
) -> Result<User, String> {
    let claims = auth_manager.validate_token(&token).await
        .map_err(|e| {
            error!("Token validation failed: {}", e);
            "Invalid token".to_string()
        })?;

    auth_manager.get_user(&claims.sub).await
        .map_err(|e| {
            error!("Failed to get user: {}", e);
            e.to_string()
        })
}

/// Add device fingerprint for user
#[command]
pub async fn add_device_fingerprint(
    auth_manager: State<'_, Arc<AuthManager>>,
    token: String,
    fingerprint: String,
) -> Result<bool, String> {
    let claims = auth_manager.validate_token(&token).await
        .map_err(|e| {
            error!("Token validation failed: {}", e);
            "Invalid token".to_string()
        })?;

    auth_manager.add_device_fingerprint(&claims.sub, &fingerprint).await
        .map_err(|e| {
            error!("Failed to add device fingerprint: {}", e);
            e.to_string()
        })?;

    info!("Device fingerprint added for user: {}", claims.sub);
    Ok(true)
}

/// Get user sessions
#[command]
pub async fn get_user_sessions(
    auth_manager: State<'_, Arc<AuthManager>>,
    token: String,
) -> Result<Vec<crate::security::auth::AuthSession>, String> {
    let claims = auth_manager.validate_token(&token).await
        .map_err(|e| {
            error!("Token validation failed: {}", e);
            "Invalid token".to_string()
        })?;

    let sessions = auth_manager.get_user_sessions(&claims.sub).await;
    Ok(sessions)
}

/// Revoke all user sessions
#[command]
pub async fn revoke_all_sessions(
    auth_manager: State<'_, Arc<AuthManager>>,
    app_state: State<'_, AppState>,
    token: String,
) -> Result<bool, String> {
    let claims = auth_manager.validate_token(&token).await
        .map_err(|e| {
            error!("Token validation failed: {}", e);
            "Invalid token".to_string()
        })?;

    auth_manager.revoke_all_user_sessions(&claims.sub).await
        .map_err(|e| {
            error!("Failed to revoke all sessions: {}", e);
            e.to_string()
        })?;

    // Clear app state
    auth_manager.clear_app_state_auth(&app_state).await
        .map_err(|e| {
            error!("Failed to clear app state after revoking sessions: {}", e);
            e.to_string()
        })?;

    info!("All sessions revoked for user: {}", claims.sub);
    Ok(true)
}

/// Generate device fingerprint
#[command]
#[instrument]
pub async fn generate_device_fingerprint() -> Result<String, String> {
    crate::security::auth::DeviceFingerprintManager::generate_fingerprint()
        .map_err(|e| {
            error!("Failed to generate device fingerprint: {}", e);
            e.to_string()
        })
}

/// Clean up expired sessions and tokens
#[command]
#[instrument(skip(auth_manager))]
pub async fn cleanup_expired_auth(
    auth_manager: State<'_, Arc<AuthManager>>,
) -> Result<bool, String> {
    info!("Cleaning up expired authentication data");
    
    auth_manager.cleanup_expired().await
        .map_err(|e| {
            error!("Failed to cleanup expired auth data: {}", e);
            e.to_string()
        })?;

    info!("Expired authentication data cleaned up successfully");
    Ok(true)
}
