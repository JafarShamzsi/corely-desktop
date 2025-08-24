import React from 'react';
import type { SystemInfo } from '@/types';

interface SystemInfoCardProps {
  systemInfo: SystemInfo | null;
  loading: boolean;
}

const SystemInfoCard: React.FC<SystemInfoCardProps> = ({ systemInfo, loading }) => {
  if (loading) {
    return (
      <div className="system-info-card">
        <h3>System Information</h3>
        <p>Loading...</p>
      </div>
    );
  }

  if (!systemInfo) {
    return (
      <div className="system-info-card">
        <h3>System Information</h3>
        <p>Failed to load system information</p>
      </div>
    );
  }

  return (
    <div className="system-info-card">
      <h3>System Information</h3>
      <div className="info-grid">
        <div className="info-item">
          <label>CPU Cores:</label>
          <span>{systemInfo.cpu_count}</span>
        </div>
        <div className="info-item">
          <label>Total Memory:</label>
          <span>{(systemInfo.total_memory / (1024 * 1024 * 1024)).toFixed(2)} GB</span>
        </div>
        <div className="info-item">
          <label>Available Memory:</label>
          <span>{(systemInfo.available_memory / (1024 * 1024 * 1024)).toFixed(2)} GB</span>
        </div>
        <div className="info-item">
          <label>Operating System:</label>
          <span>{systemInfo.os_type}</span>
        </div>
      </div>
    </div>
  );
};

export default SystemInfoCard;
