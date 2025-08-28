import React from 'react';
import type { SystemInfo } from '@/types';

interface SystemInfoCardProps {
  systemInfo: SystemInfo | null;
  loading: boolean;
}

const SystemInfoCard: React.FC<SystemInfoCardProps> = ({ systemInfo, loading }) => {
  if (loading) {
    return (
      <div className="info-card">
        <div className="card-header">
          <h3>System Information</h3>
          <div className="loading-spinner-small"></div>
        </div>
        <div className="system-info">
          <div className="info-row loading-shimmer">
            <span className="label">Loading...</span>
            <span className="value">Please wait</span>
          </div>
        </div>
      </div>
    );
  }

  if (!systemInfo) {
    return (
      <div className="info-card">
        <div className="card-header">
          <h3>System Information</h3>
          <span className="error-badge">Error</span>
        </div>
        <div className="system-info">
          <div className="info-row">
            <span className="label">Status:</span>
            <span className="value error">Failed to load</span>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="info-card">
      <div className="card-header">
        <h3>System Information</h3>
        <span className="status-badge online">Active</span>
      </div>
      <div className="system-info">
        <div className="info-row">
          <span className="label">CPU Cores:</span>
          <span className="value">{systemInfo.cpu_count}</span>
        </div>
        <div className="info-row">
          <span className="label">Total Memory:</span>
          <span className="value">{(systemInfo.total_memory / (1024 * 1024 * 1024)).toFixed(1)} GB</span>
        </div>
        <div className="info-row">
          <span className="label">Available Memory:</span>
          <span className="value">{(systemInfo.available_memory / (1024 * 1024 * 1024)).toFixed(1)} GB</span>
        </div>
        <div className="info-row">
          <span className="label">Operating System:</span>
          <span className="value">{systemInfo.os_type}</span>
        </div>
        {systemInfo.gpu_info && systemInfo.gpu_info.length > 0 && (
          <div className="info-row">
            <span className="label">GPU:</span>
            <span className="value">{systemInfo.gpu_info[0]}</span>
          </div>
        )}
      </div>
    </div>
  );
};

export default SystemInfoCard;
