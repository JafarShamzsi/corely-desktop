import React, { useState, useEffect } from 'react';
import { useAuth } from '@/hooks/useAuth';
import SystemInfoCard from '@/components/SystemInfoCard';
import ModelList from '@/components/ModelList';
import { systemService, mlService } from '@/services/api';
import { authService } from '@/services/authService';
import type { SystemInfo, MLModel } from '@/types';
import './MainDashboard.css';

const MainDashboard: React.FC = () => {
  const { user, session, logout } = useAuth();
  const [systemInfo, setSystemInfo] = useState<SystemInfo | null>(null);
  const [models, setModels] = useState<MLModel[]>([]);
  const [systemLoading, setSystemLoading] = useState(true);
  const [modelsLoading, setModelsLoading] = useState(true);
  const [activeSessions, setActiveSessions] = useState<any[]>([]);
  const [deviceFingerprint, setDeviceFingerprint] = useState<string>('');

  // Real-time data refresh interval
  useEffect(() => {
    loadInitialData();
    
    // Set up real-time updates every 10 seconds
    const interval = setInterval(() => {
      loadSystemInfo();
      loadAuthInfo();
    }, 10000);

    return () => clearInterval(interval);
  }, []);

  const loadInitialData = async () => {
    await Promise.all([
      loadSystemInfo(),
      loadModels(),
      loadAuthInfo()
    ]);
  };

  const loadSystemInfo = async () => {
    try {
      setSystemLoading(true);
      const info = await systemService.getSystemInfo();
      setSystemInfo(info);
    } catch (error) {
      console.error('Failed to load system info:', error);
    } finally {
      setSystemLoading(false);
    }
  };

  const loadModels = async () => {
    try {
      setModelsLoading(true);
      const modelList = await mlService.listAvailableModels();
      setModels(modelList);
    } catch (error) {
      console.error('Failed to load models:', error);
    } finally {
      setModelsLoading(false);
    }
  };

  const loadAuthInfo = async () => {
    try {
      const [sessions, fingerprint] = await Promise.all([
        authService.getActiveSessions(),
        authService.getDeviceFingerprint()
      ]);
      setActiveSessions(sessions);
      setDeviceFingerprint(fingerprint);
    } catch (error) {
      console.error('Failed to load auth info:', error);
    }
  };

  const handleLoadModel = async (modelId: string) => {
    try {
      const success = await mlService.loadModel(modelId);
      if (success) {
        await loadModels(); // Refresh models list
      }
    } catch (error) {
      console.error('Failed to load model:', error);
    }
  };

  const handleLogout = async () => {
    if (confirm('Are you sure you want to log out?')) {
      await logout();
    }
  };

  const formatUptime = (seconds: number): string => {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const remainingSeconds = seconds % 60;
    return `${hours}h ${minutes}m ${remainingSeconds}s`;
  };

  const getSessionExpiryTime = (): string => {
    if (!session) return 'Unknown';
    const expiresAt = new Date(session.expires_at);
    const now = new Date();
    const timeLeft = expiresAt.getTime() - now.getTime();
    
    if (timeLeft <= 0) return 'Expired';
    
    const hours = Math.floor(timeLeft / (1000 * 60 * 60));
    const minutes = Math.floor((timeLeft % (1000 * 60 * 60)) / (1000 * 60));
    
    return `${hours}h ${minutes}m`;
  };

  return (
    <div className="main-dashboard">
      {/* Header */}
      <header className="dashboard-header">
        <div className="header-left">
          <h1>AI Training Marketplace</h1>
          <p className="subtitle">Desktop Application Dashboard</p>
        </div>
        <div className="header-right">
          <div className="user-info">
            <div className="user-avatar">
              {user?.username?.charAt(0).toUpperCase() || 'U'}
            </div>
            <div className="user-details">
              <span className="username">{user?.username || 'User'}</span>
              <span className="user-email">{user?.email || 'No email'}</span>
            </div>
            <button onClick={handleLogout} className="logout-btn">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor">
                <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"/>
                <polyline points="16,17 21,12 16,7"/>
                <line x1="21" y1="12" x2="9" y2="12"/>
              </svg>
              Logout
            </button>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="dashboard-content">
        {/* Stats Overview */}
        <section className="stats-overview">
          <div className="stat-card">
            <div className="stat-icon">🖥️</div>
            <div className="stat-info">
              <h3>System Status</h3>
              <p className={`stat-value ${systemInfo ? 'online' : 'loading'}`}>
                {systemLoading ? 'Loading...' : systemInfo ? 'Online' : 'Offline'}
              </p>
            </div>
          </div>

          <div className="stat-card">
            <div className="stat-icon">🤖</div>
            <div className="stat-info">
              <h3>Available Models</h3>
              <p className="stat-value">
                {modelsLoading ? 'Loading...' : models.length}
              </p>
            </div>
          </div>

          <div className="stat-card">
            <div className="stat-icon">🔐</div>
            <div className="stat-info">
              <h3>Session Expires</h3>
              <p className="stat-value">{getSessionExpiryTime()}</p>
            </div>
          </div>

          <div className="stat-card">
            <div className="stat-icon">📱</div>
            <div className="stat-info">
              <h3>Active Sessions</h3>
              <p className="stat-value">{activeSessions.length}</p>
            </div>
          </div>
        </section>

        {/* Main Grid */}
        <div className="dashboard-grid">
          {/* System Information */}
          <div className="grid-item">
            <SystemInfoCard 
              systemInfo={systemInfo} 
              loading={systemLoading}
            />
          </div>

          {/* Model Management */}
          <div className="grid-item">
            <ModelList 
              models={models}
              loading={modelsLoading}
              onLoadModel={handleLoadModel}
            />
          </div>

          {/* Authentication Info */}
          <div className="grid-item">
            <div className="info-card">
              <div className="card-header">
                <h3>Authentication Details</h3>
                <button onClick={loadAuthInfo} className="refresh-btn">
                  <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor">
                    <polyline points="23,4 23,10 17,10"/>
                    <path d="M20.49,15A9,9,0,1,1,5.64,5.64L23,10"/>
                  </svg>
                </button>
              </div>
              <div className="auth-info">
                <div className="info-row">
                  <span className="label">User ID:</span>
                  <span className="value">{user?.id || 'N/A'}</span>
                </div>
                <div className="info-row">
                  <span className="label">Login Method:</span>
                  <span className="value">
                    {user?.oauth_provider ? 
                      `OAuth2 (${user.oauth_provider})` : 
                      'Username/Password'
                    }
                  </span>
                </div>
                <div className="info-row">
                  <span className="label">Session ID:</span>
                  <span className="value session-id">
                    {session?.session_id?.substring(0, 8) || 'N/A'}...
                  </span>
                </div>
                <div className="info-row">
                  <span className="label">Device Fingerprint:</span>
                  <span className="value fingerprint">
                    {deviceFingerprint.substring(0, 12) || 'Loading'}...
                  </span>
                </div>
                <div className="info-row">
                  <span className="label">Last Login:</span>
                  <span className="value">
                    {user?.last_login ? 
                      new Date(user.last_login).toLocaleString() : 
                      'N/A'
                    }
                  </span>
                </div>
                <div className="info-row">
                  <span className="label">Created:</span>
                  <span className="value">
                    {user?.created_at ? 
                      new Date(user.created_at).toLocaleDateString() : 
                      'N/A'
                    }
                  </span>
                </div>
              </div>
            </div>
          </div>

          {/* Real-time Status */}
          <div className="grid-item">
            <div className="info-card">
              <div className="card-header">
                <h3>Real-time Status</h3>
                <div className="status-indicator online">
                  <span className="indicator-dot"></span>
                  Live
                </div>
              </div>
              <div className="status-info">
                <div className="info-row">
                  <span className="label">CPU Usage:</span>
                  <span className="value">
                    {systemInfo ? 'Active' : 'Loading...'}
                  </span>
                </div>
                <div className="info-row">
                  <span className="label">Memory:</span>
                  <span className="value">
                    {systemInfo ? 
                      `${(systemInfo.total_memory / 1024 / 1024 / 1024).toFixed(1)}GB Total` : 
                      'Loading...'
                    }
                  </span>
                </div>
                <div className="info-row">
                  <span className="label">GPU:</span>
                  <span className="value">
                    {systemInfo?.gpu_info?.length ? systemInfo.gpu_info[0] : 'No GPU detected'}
                  </span>
                </div>
                <div className="info-row">
                  <span className="label">Platform:</span>
                  <span className="value">
                    {systemInfo?.os_type || 'Unknown'}
                  </span>
                </div>
                <div className="info-row">
                  <span className="label">App Version:</span>
                  <span className="value">v1.0.0</span>
                </div>
                <div className="info-row">
                  <span className="label">Last Updated:</span>
                  <span className="value">
                    {new Date().toLocaleTimeString()}
                  </span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </main>
    </div>
  );
};

export default MainDashboard;
