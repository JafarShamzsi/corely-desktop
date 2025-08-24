import React from 'react';
import SystemInfoCard from '@/components/SystemInfoCard';
import ModelList from '@/components/ModelList';
import { systemService, mlService } from '@/services/api';
import type { SystemInfo, MLModel } from '@/types';

const Dashboard: React.FC = () => {
  const [systemInfo, setSystemInfo] = React.useState<SystemInfo | null>(null);
  const [models, setModels] = React.useState<MLModel[]>([]);
  const [systemLoading, setSystemLoading] = React.useState(true);
  const [modelsLoading, setModelsLoading] = React.useState(true);

  React.useEffect(() => {
    loadSystemInfo();
    loadModels();
  }, []);

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

  const handleLoadModel = async (modelId: string) => {
    try {
      const success = await mlService.loadModel(modelId);
      if (success) {
        // Refresh models to update the loaded status
        await loadModels();
      }
    } catch (error) {
      console.error('Failed to load model:', error);
    }
  };

  return (
    <div className="dashboard">
      <h1>AI Training Desktop</h1>
      <div className="dashboard-grid">
        <SystemInfoCard systemInfo={systemInfo} loading={systemLoading} />
        <ModelList 
          models={models} 
          loading={modelsLoading} 
          onLoadModel={handleLoadModel} 
        />
      </div>
    </div>
  );
};

export default Dashboard;
