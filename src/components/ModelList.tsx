import React from 'react';
import type { MLModel } from '@/types';

interface ModelListProps {
  models: MLModel[];
  loading: boolean;
  onLoadModel: (modelId: string) => void;
}

const ModelList: React.FC<ModelListProps> = ({ models, loading, onLoadModel }) => {
  if (loading) {
    return (
      <div className="info-card">
        <div className="card-header">
          <h3>ML Models</h3>
          <div className="loading-spinner-small"></div>
        </div>
        <div className="model-info">
          <div className="info-row loading-shimmer">
            <span className="label">Loading models...</span>
            <span className="value">Please wait</span>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="info-card">
      <div className="card-header">
        <h3>ML Models ({models.length})</h3>
        <span className={`status-badge ${models.length > 0 ? 'online' : 'offline'}`}>
          {models.length > 0 ? 'Available' : 'None'}
        </span>
      </div>
      <div className="model-info">
        {models.length === 0 ? (
          <div className="info-row">
            <span className="label">Status:</span>
            <span className="value">No models available</span>
          </div>
        ) : (
          <div className="models-list">
            {models.map((model) => (
              <div key={model.id} className="model-item">
                <div className="model-details">
                  <div className="model-name">{model.name}</div>
                  <div className="model-meta">
                    <span className="framework">{model.framework}</span>
                    <span className="size">{model.size_mb} MB</span>
                  </div>
                </div>
                <div className="model-actions">
                  <span className={`status-indicator ${model.is_loaded ? 'loaded' : 'unloaded'}`}>
                    {model.is_loaded ? 'Loaded' : 'Available'}
                  </span>
                  <button
                    onClick={() => onLoadModel(model.id)}
                    disabled={model.is_loaded}
                    className={`model-button ${model.is_loaded ? 'loaded' : 'load'}`}
                  >
                    {model.is_loaded ? '✓ Loaded' : 'Load'}
                  </button>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
};

export default ModelList;
