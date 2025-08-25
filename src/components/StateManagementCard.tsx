import React, { useState, useEffect } from 'react';
import { stateService } from '@/services/api';
import type { SetupStatus, HardwareProfile, AuthState, TrainingState, AppConfig, AppEvent } from '@/types';
import './StateManagementCard.css';

const StateManagementCard: React.FC = () => {
  const [setupStatus, setSetupStatus] = useState<SetupStatus | null>(null);
  const [hardwareProfile, setHardwareProfile] = useState<HardwareProfile | null>(null);
  const [authState, setAuthState] = useState<AuthState | null>(null);
  const [trainingState, setTrainingState] = useState<TrainingState | null>(null);
  const [appConfig, setAppConfig] = useState<AppConfig | null>(null);
  const [events, setEvents] = useState<AppEvent[]>([]);
  const [isReady, setIsReady] = useState<boolean>(false);
  const [uptime, setUptime] = useState<number>(0);
  const [loading, setLoading] = useState(true);
  
  // Auth form state
  const [username, setUsername] = useState('demo_user');
  const [password, setPassword] = useState('demo_pass');
  const [loginLoading, setLoginLoading] = useState(false);
  
  // Training form state
  const [modelName, setModelName] = useState('resnet50_demo');
  const [trainingLoading, setTrainingLoading] = useState(false);

  useEffect(() => {
    loadAllState();
    
    // Set up periodic refresh
    const interval = setInterval(() => {
      loadTrainingState();
      loadUptime();
      loadEvents();
    }, 2000);
    
    return () => clearInterval(interval);
  }, []);

  const loadAllState = async () => {
    try {
      setLoading(true);
      const [
        statusResponse,
        hardwareResponse,
        authResponse,
        trainingResponse,
        configResponse,
        readyResponse,
        uptimeResponse,
        eventsResponse
      ] = await Promise.all([
        stateService.getSetupStatus().catch(() => null),
        stateService.getHardwareProfile().catch(() => null),
        stateService.getAuthState().catch(() => null),
        stateService.getTrainingState().catch(() => null),
        stateService.getAppConfig().catch(() => null),
        stateService.isAppReady().catch(() => false),
        stateService.getAppUptime().catch(() => 0),
        stateService.getEventHistory(10).catch(() => [])
      ]);
      
      setSetupStatus(statusResponse);
      setHardwareProfile(hardwareResponse);
      setAuthState(authResponse);
      setTrainingState(trainingResponse);
      setAppConfig(configResponse);
      setIsReady(readyResponse);
      setUptime(uptimeResponse);
      setEvents(eventsResponse);
    } catch (error) {
      console.error('Failed to load state:', error);
    } finally {
      setLoading(false);
    }
  };

  const loadTrainingState = async () => {
    try {
      const state = await stateService.getTrainingState();
      setTrainingState(state);
    } catch (error) {
      console.error('Failed to load training state:', error);
    }
  };

  const loadUptime = async () => {
    try {
      const time = await stateService.getAppUptime();
      setUptime(time);
    } catch (error) {
      console.error('Failed to load uptime:', error);
    }
  };

  const loadEvents = async () => {
    try {
      const eventHistory = await stateService.getEventHistory(10);
      setEvents(eventHistory);
    } catch (error) {
      console.error('Failed to load events:', error);
    }
  };

  const handleLogin = async () => {
    try {
      setLoginLoading(true);
      const auth = await stateService.authenticateUser(username, password);
      setAuthState(auth);
      await loadEvents(); // Refresh events after login
    } catch (error) {
      console.error('Login failed:', error);
      alert('Login failed: ' + error);
    } finally {
      setLoginLoading(false);
    }
  };

  const handleLogout = async () => {
    try {
      await stateService.logoutUser();
      setAuthState(prev => prev ? { ...prev, is_authenticated: false } : null);
      await loadEvents(); // Refresh events after logout
    } catch (error) {
      console.error('Logout failed:', error);
    }
  };

  const handleStartTraining = async () => {
    try {
      setTrainingLoading(true);
      const sessionId = await stateService.startTrainingSession(modelName);
      console.log('Training started with session ID:', sessionId);
      await loadTrainingState();
      await loadEvents();
    } catch (error) {
      console.error('Failed to start training:', error);
      alert('Failed to start training: ' + error);
    } finally {
      setTrainingLoading(false);
    }
  };

  const handleStopTraining = async () => {
    try {
      setTrainingLoading(true);
      await stateService.stopTrainingSession();
      await loadTrainingState();
      await loadEvents();
    } catch (error) {
      console.error('Failed to stop training:', error);
    } finally {
      setTrainingLoading(false);
    }
  };

  const formatBytes = (bytes: number) => {
    const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];
    if (bytes === 0) return '0 Bytes';
    const i = Math.floor(Math.log(bytes) / Math.log(1024));
    return Math.round(bytes / Math.pow(1024, i) * 100) / 100 + ' ' + sizes[i];
  };

  const formatDuration = (seconds: number) => {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const secs = seconds % 60;
    return `${hours}h ${minutes}m ${secs}s`;
  };

  const getSetupProgress = (status: SetupStatus | null) => {
    if (!status) return 0;
    
    switch (status.type) {
      case 'SystemAnalysis': return Math.floor(status.progress / 4);
      case 'HardwareDetection': return 25 + Math.floor(status.progress / 4);
      case 'DriverCheck': return 50 + Math.floor(status.progress / 4);
      case 'EnvironmentSetup': return 75 + Math.floor(status.progress / 4);
      case 'Complete': return 100;
      case 'Failed': return 0;
      default: return 0;
    }
  };

  if (loading) {
    return (
      <div className="state-management-card">
        <h2>🔄 Loading State Management System...</h2>
      </div>
    );
  }

  return (
    <div className="state-management-card">
      <div className="card-header">
        <h2>🚀 AI Training Marketplace - State Management</h2>
        <div className="status-indicators">
          <span className={`status-badge ${isReady ? 'ready' : 'not-ready'}`}>
            {isReady ? '✅ Ready' : '⏳ Initializing'}
          </span>
          <span className="uptime">⏱️ Uptime: {formatDuration(uptime)}</span>
        </div>
      </div>

      <div className="state-grid">
        {/* Setup Status */}
        <div className="state-section">
          <h3>📋 Setup Status</h3>
          {setupStatus ? (
            <div className="setup-status">
              <div className="progress-bar">
                <div 
                  className="progress-fill" 
                  style={{ width: `${getSetupProgress(setupStatus)}%` }}
                />
                <span className="progress-text">{getSetupProgress(setupStatus)}%</span>
              </div>
              <div className="status-details">
                <p><strong>Stage:</strong> {setupStatus.type}</p>
                {setupStatus.type === 'SystemAnalysis' && (
                  <p><strong>Details:</strong> {setupStatus.details}</p>
                )}
                {setupStatus.type === 'Complete' && (
                  <p><strong>Summary:</strong> {setupStatus.summary}</p>
                )}
                {setupStatus.type === 'Failed' && (
                  <p className="error"><strong>Error:</strong> {setupStatus.error}</p>
                )}
              </div>
            </div>
          ) : (
            <p>No setup status available</p>
          )}
        </div>

        {/* Hardware Profile */}
        <div className="state-section">
          <h3>💻 Hardware Profile</h3>
          {hardwareProfile ? (
            <div className="hardware-info">
              <div className="hardware-item">
                <strong>CPU:</strong> {hardwareProfile.cpu.name}
                <br />
                <small>{hardwareProfile.cpu.cores} cores, {hardwareProfile.cpu.threads} threads</small>
              </div>
              {hardwareProfile.gpu && (
                <div className="hardware-item">
                  <strong>GPU:</strong> {hardwareProfile.gpu.name}
                  <br />
                  <small>{formatBytes(hardwareProfile.gpu.memory)} VRAM</small>
                </div>
              )}
              <div className="hardware-item">
                <strong>Memory:</strong> {formatBytes(hardwareProfile.memory.total)}
                <br />
                <small>{formatBytes(hardwareProfile.memory.available)} available</small>
              </div>
              <div className="hardware-item">
                <strong>Storage:</strong> {formatBytes(hardwareProfile.storage.total_space)}
                <br />
                <small>{formatBytes(hardwareProfile.storage.available_space)} free</small>
              </div>
            </div>
          ) : (
            <p>No hardware profile detected</p>
          )}
        </div>

        {/* Authentication */}
        <div className="state-section">
          <h3>🔐 Authentication</h3>
          {authState?.is_authenticated ? (
            <div className="auth-info">
              <p><strong>User:</strong> {authState.username}</p>
              <p><strong>Email:</strong> {authState.email}</p>
              <p><strong>Permissions:</strong> {authState.permissions.join(', ')}</p>
              <button onClick={handleLogout} className="btn btn-secondary">
                Logout
              </button>
            </div>
          ) : (
            <div className="auth-form">
              <div className="form-group">
                <input
                  type="text"
                  placeholder="Username"
                  value={username}
                  onChange={(e) => setUsername(e.target.value)}
                />
              </div>
              <div className="form-group">
                <input
                  type="password"
                  placeholder="Password"
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                />
              </div>
              <button 
                onClick={handleLogin} 
                disabled={loginLoading}
                className="btn btn-primary"
              >
                {loginLoading ? 'Logging in...' : 'Login'}
              </button>
            </div>
          )}
        </div>

        {/* Training State */}
        <div className="state-section">
          <h3>🤖 Training Session</h3>
          {trainingState ? (
            <div className="training-info">
              {trainingState.is_training ? (
                <div className="training-active">
                  <p><strong>Status:</strong> 🟢 Training Active</p>
                  <p><strong>Model:</strong> {trainingState.current_model}</p>
                  <p><strong>Session ID:</strong> {trainingState.current_session_id}</p>
                  <p><strong>Progress:</strong> {Math.round(trainingState.progress * 100)}%</p>
                  <p><strong>Epoch:</strong> {trainingState.epoch}</p>
                  <button 
                    onClick={handleStopTraining} 
                    disabled={trainingLoading}
                    className="btn btn-danger"
                  >
                    {trainingLoading ? 'Stopping...' : 'Stop Training'}
                  </button>
                </div>
              ) : (
                <div className="training-idle">
                  <p><strong>Status:</strong> ⭕ No Active Training</p>
                  <div className="form-group">
                    <input
                      type="text"
                      placeholder="Model name"
                      value={modelName}
                      onChange={(e) => setModelName(e.target.value)}
                    />
                  </div>
                  <button 
                    onClick={handleStartTraining} 
                    disabled={trainingLoading || !authState?.is_authenticated}
                    className="btn btn-primary"
                  >
                    {trainingLoading ? 'Starting...' : 'Start Training'}
                  </button>
                  {!authState?.is_authenticated && (
                    <p className="warning">⚠️ Please login to start training</p>
                  )}
                </div>
              )}
            </div>
          ) : (
            <p>No training state available</p>
          )}
        </div>

        {/* Configuration */}
        <div className="state-section">
          <h3>⚙️ Configuration</h3>
          {appConfig ? (
            <div className="config-info">
              <p><strong>API Endpoint:</strong> {appConfig.api_endpoint}</p>
              <p><strong>Max Memory:</strong> {appConfig.max_memory_usage_gb} GB</p>
              <p><strong>Distributed Training:</strong> {appConfig.enable_distributed_training ? '✅' : '❌'}</p>
              <p><strong>Log Level:</strong> {appConfig.log_level}</p>
              <p><strong>Telemetry:</strong> {appConfig.enable_telemetry ? '✅' : '❌'}</p>
            </div>
          ) : (
            <p>No configuration available</p>
          )}
        </div>

        {/* Event History */}
        <div className="state-section">
          <h3>📝 Recent Events</h3>
          <div className="events-list">
            {events.length > 0 ? (
              events.map((event, index) => (
                <div key={index} className="event-item">
                  <span className="event-type">{event.type}</span>
                  <span className="event-time">
                    {new Date(event.timestamp).toLocaleTimeString()}
                  </span>
                </div>
              ))
            ) : (
              <p>No recent events</p>
            )}
          </div>
        </div>
      </div>
    </div>
  );
};

export default StateManagementCard;
