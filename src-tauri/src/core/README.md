# Core Application State Management System

## Overview

This is a comprehensive, thread-safe state management system for the AI Training Marketplace desktop application. It provides centralized state management with real-time event notifications, hardware profiling, authentication, and setup tracking.

## Features

### 🏗️ **Modular Architecture**
- **Thread-safe**: Uses `Arc<RwLock<T>>` and `Arc<Mutex<T>>` for safe concurrent access
- **Event-driven**: Real-time updates between Rust backend and React frontend
- **Type-safe**: Comprehensive error handling with custom error types
- **Scalable**: Designed to handle complex state transitions and multiple subsystems

### 📊 **State Components**

#### 1. **Setup Status Tracking**
```rust
pub enum SetupStatus {
    SystemAnalysis { progress: u8, details: String, started_at: DateTime<Utc> },
    HardwareDetection { progress: u8, cpu_detected: bool, gpu_detected: bool, memory_detected: bool, started_at: DateTime<Utc> },
    DriverCheck { progress: u8, drivers_status: HashMap<String, bool>, recommendations: Vec<String>, started_at: DateTime<Utc> },
    EnvironmentSetup { progress: u8, dependencies_installed: Vec<String>, environment_ready: bool, started_at: DateTime<Utc> },
    Complete { completed_at: DateTime<Utc>, summary: String },
    Failed { error: String, failed_at: DateTime<Utc>, retry_count: u32 },
}
```

#### 2. **Hardware Profile Management**
```rust
pub struct HardwareProfile {
    pub cpu: CpuInfo,
    pub gpu: Option<GpuInfo>,
    pub memory: MemoryInfo,
    pub storage: StorageInfo,
    pub last_updated: DateTime<Utc>,
}
```

#### 3. **Authentication State**
```rust
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
```

#### 4. **Training Session Management**
```rust
pub struct TrainingState {
    pub is_training: bool,
    pub current_session_id: Option<String>,
    pub current_model: Option<String>,
    pub progress: f64,
    pub epoch: u32,
    pub batch: u32,
    pub loss: Option<f64>,
    pub accuracy: Option<f64>,
    pub training_start: Option<DateTime<Utc>>,
    pub estimated_completion: Option<DateTime<Utc>>,
}
```

### 🔄 **Real-time Event System**

The system emits events for all state changes:

```rust
pub enum AppEvent {
    SetupStatusChanged { old_status: SetupStatus, new_status: SetupStatus, timestamp: DateTime<Utc> },
    HardwareProfileUpdated { profile: HardwareProfile, timestamp: DateTime<Utc> },
    AuthStateChanged { authenticated: bool, user_id: Option<String>, timestamp: DateTime<Utc> },
    TrainingSessionChanged { session_id: Option<String>, model_name: Option<String>, status: String, timestamp: DateTime<Utc> },
    ResourceUsageUpdated { cpu_usage: f64, memory_usage: f64, gpu_usage: Option<f64>, timestamp: DateTime<Utc> },
    ErrorOccurred { error: StateError, context: String, timestamp: DateTime<Utc> },
}
```

## Usage Examples

### Basic State Initialization

```rust
use crate::core::AppState;

// Initialize the application state
let state = AppState::new();

// Add event listener for real-time updates
state.add_event_listener(|event| {
    println!("State changed: {:?}", event);
})?;
```

### Setup Process Management

```rust
// Update setup status
state.update_setup_status(SetupStatus::HardwareDetection {
    progress: 50,
    cpu_detected: true,
    gpu_detected: true,
    memory_detected: true,
    started_at: Utc::now(),
})?;

// Check if setup is complete
if state.is_ready()? {
    println!("Application ready for use!");
}
```

### Training Session Management

```rust
// Start a training session
let session_id = state.start_training("resnet50_custom".to_string())?;
println!("Training started with session ID: {}", session_id);

// Get current training state
let training_state = state.get_training_state()?;
if training_state.is_training {
    println!("Training in progress: {}%", training_state.progress * 100.0);
}

// Stop training
state.stop_training()?;
```

### Authentication Management

```rust
// Update authentication state
let auth_state = AuthState {
    is_authenticated: true,
    user_id: Some("user_123".to_string()),
    username: Some("researcher".to_string()),
    token: Some("jwt_token".to_string()),
    permissions: vec!["read".to_string(), "write".to_string(), "train".to_string()],
    ..Default::default()
};

state.update_auth_state(auth_state)?;

// Check authentication
let current_auth = state.get_auth_state()?;
if current_auth.is_authenticated {
    println!("User logged in: {:?}", current_auth.username);
}
```

## Tauri Integration

### Available Commands

The system provides these Tauri commands for frontend integration:

```typescript
// State retrieval commands
invoke('get_setup_status'): Promise<SetupStatus>
invoke('get_hardware_profile'): Promise<HardwareProfile | null>
invoke('get_auth_state'): Promise<AuthState>
invoke('get_training_state'): Promise<TrainingState>
invoke('get_app_config'): Promise<AppConfig>

// State modification commands
invoke('update_app_config', { config }): Promise<void>
invoke('start_training_session', { modelName }): Promise<string>
invoke('stop_training_session'): Promise<void>
invoke('authenticate_user', { username, password }): Promise<AuthState>
invoke('logout_user'): Promise<void>

// Utility commands
invoke('is_app_ready'): Promise<boolean>
invoke('get_app_uptime'): Promise<number>
invoke('get_event_history', { limit? }): Promise<AppEvent[]>
```

### Frontend Integration Example

```typescript
import { invoke } from '@tauri-apps/api/tauri';

// Check application readiness
const isReady = await invoke<boolean>('is_app_ready');
if (isReady) {
    console.log('Application is ready!');
}

// Get current setup status
const setupStatus = await invoke<SetupStatus>('get_setup_status');
console.log('Setup progress:', setupStatus.progress);

// Start training session
try {
    const sessionId = await invoke<string>('start_training_session', {
        modelName: 'my_custom_model'
    });
    console.log('Training started:', sessionId);
} catch (error) {
    console.error('Failed to start training:', error);
}
```

## Error Handling

The system uses comprehensive error types:

```rust
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
```

## Logging and Debugging

The system integrates with the `tracing` crate for structured logging:

```rust
use tracing::{info, warn, error, instrument};

#[instrument(skip(self))]
pub fn update_setup_status(&self, new_status: SetupStatus) -> Result<(), StateError> {
    info!("Setup status updated: {:?}", new_status);
    // ... implementation
}
```

## Testing

The system includes comprehensive tests:

```bash
# Run all tests
cargo test

# Run state management tests specifically
cargo test --test state_tests

# Run with output
cargo test -- --nocapture
```

## Thread Safety

All state components are designed for safe concurrent access:

- **Read operations**: Use `RwLock::read()` for non-blocking concurrent reads
- **Write operations**: Use `RwLock::write()` for exclusive write access
- **Event system**: Uses `Mutex` for thread-safe event listener management
- **Event history**: Protected by `Mutex` for safe concurrent access

## Performance Considerations

### Event History Management
- Automatically limits history to 1000 events to prevent memory bloat
- Events are cloned for listeners to avoid blocking
- Lock acquisition is minimized with scoped access

### Lock Strategy
- Uses `RwLock` for data that's read frequently but written rarely
- Uses `Mutex` for data with mixed read/write patterns
- Instruments all lock acquisitions for debugging

### Memory Usage
- Clones only necessary data for event emission
- Uses `Arc` for shared ownership without copying large structures
- Regularly cleans up old events and expired sessions

## Architecture Benefits

1. **Type Safety**: Compile-time guarantees for state consistency
2. **Concurrency**: Safe multi-threaded access without data races
3. **Observability**: Complete audit trail of state changes
4. **Modularity**: Clear separation of concerns across components
5. **Testability**: Isolated components with mocked dependencies
6. **Performance**: Optimized for high-frequency state updates
7. **Maintainability**: Self-documenting code with comprehensive error handling

## Future Enhancements

- **Persistence**: Save/restore state to disk
- **State Snapshots**: Create immutable state snapshots for debugging
- **Metrics**: Integration with monitoring systems
- **State Validation**: Runtime validation of state transitions
- **Hot Reloading**: Dynamic state schema updates

---

This state management system provides a solid foundation for the AI Training Marketplace desktop application, ensuring reliable, performant, and maintainable state handling across all application components.
