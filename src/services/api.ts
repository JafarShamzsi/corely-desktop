import { invoke } from '@tauri-apps/api/core';
import type { SystemInfo, MLModel, NetworkNode } from '@/types';

export const systemService = {
  async getSystemInfo(): Promise<SystemInfo> {
    return await invoke<SystemInfo>('get_system_info');
  },
};

export const mlService = {
  async listAvailableModels(): Promise<MLModel[]> {
    return await invoke<MLModel[]>('list_available_models');
  },

  async loadModel(modelId: string): Promise<boolean> {
    return await invoke<boolean>('load_model', { modelId });
  },
};

export const networkService = {
  async discoverNetworkNodes(): Promise<NetworkNode[]> {
    return await invoke<NetworkNode[]>('discover_network_nodes');
  },

  async connectToFlowerServer(serverAddress: string): Promise<boolean> {
    return await invoke<boolean>('connect_to_flower_server', { serverAddress });
  },
};
