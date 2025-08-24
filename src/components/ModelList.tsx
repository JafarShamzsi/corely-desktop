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
      <div className="model-list">
        <h3>Available ML Models</h3>
        <p>Loading models...</p>
      </div>
    );
  }

  return (
    <div className="model-list">
      <h3>Available ML Models</h3>
      {models.length === 0 ? (
        <p>No models available</p>
      ) : (
        <div className="models-grid">
          {models.map((model) => (
            <div key={model.id} className="model-card">
              <h4>{model.name}</h4>
              <p className="model-framework">Framework: {model.framework}</p>
              <p className="model-size">Size: {model.size_mb} MB</p>
              <p className="model-status">
                Status: {model.is_loaded ? 'Loaded' : 'Not Loaded'}
              </p>
              <button
                onClick={() => onLoadModel(model.id)}
                disabled={model.is_loaded}
                className={`load-button ${model.is_loaded ? 'loaded' : ''}`}
              >
                {model.is_loaded ? 'Loaded' : 'Load Model'}
              </button>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

export default ModelList;
