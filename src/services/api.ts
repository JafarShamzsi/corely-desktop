import { invoke } from '@tauri-apps/api/core';
import type { SystemInfo, MLModel, NetworkNode, SetupStatus, HardwareProfile, AuthState, TrainingState, AppConfig, AppEvent } from '@/types';

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

// New State Management Services
export const stateService = {
  async getSetupStatus(): Promise<SetupStatus> {
    return await invoke<SetupStatus>('get_setup_status');
  },

  async getHardwareProfile(): Promise<HardwareProfile | null> {
    return await invoke<HardwareProfile | null>('get_hardware_profile');
  },

  async getAuthState(): Promise<AuthState> {
    return await invoke<AuthState>('get_auth_state');
  },

  async getTrainingState(): Promise<TrainingState> {
    return await invoke<TrainingState>('get_training_state');
  },

  async getAppConfig(): Promise<AppConfig> {
    return await invoke<AppConfig>('get_app_config');
  },

  async updateAppConfig(config: AppConfig): Promise<void> {
    return await invoke<void>('update_app_config', { config });
  },

  async startTrainingSession(modelName: string): Promise<string> {
    return await invoke<string>('start_training_session', { modelName });
  },

  async stopTrainingSession(): Promise<void> {
    return await invoke<void>('stop_training_session');
  },

  async authenticateUser(username: string, password: string): Promise<AuthState> {
    return await invoke<AuthState>('authenticate_user', { username, password });
  },

  async logoutUser(): Promise<void> {
    return await invoke<void>('logout_user');
  },

  async isAppReady(): Promise<boolean> {
    return await invoke<boolean>('is_app_ready');
  },

  async getAppUptime(): Promise<number> {
    return await invoke<number>('get_app_uptime');
  },

  async getEventHistory(limit?: number): Promise<AppEvent[]> {
    return await invoke<AppEvent[]>('get_event_history', { limit });
  },
};
