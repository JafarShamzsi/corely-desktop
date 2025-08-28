import { invoke } from '@tauri-apps/api/core';
import type { SystemInfo, MLModel, NetworkNode, SetupStatus, HardwareProfile, AuthState, TrainingState, AppConfig, AppEvent } from '@/types';

// Check if we're running in Tauri context
const isTauriContext = () => {
  return typeof window !== 'undefined' && (window as any).__TAURI__ !== undefined;
};

// Mock invoke function for development in browser
const mockInvoke = async <T>(command: string, args?: any): Promise<T> => {
  console.warn(`Mock Tauri invoke called: ${command}`, args);
  
  // Return mock data for development
  switch (command) {
    case 'get_system_info':
      return {
        cpu_count: 8,
        total_memory: 16384,
        available_memory: 8192,
        os_type: 'Windows',
        gpu_info: ['NVIDIA GeForce RTX 4060']
      } as T;
    case 'list_available_models':
      return [] as T;
    case 'discover_network_nodes':
      return [] as T;
    case 'load_model':
    case 'connect_to_flower_server':
      return true as T;
    default:
      throw new Error(`Tauri context not available. Command: ${command}`);
  }
};

// Safe invoke that works in both Tauri and browser contexts
const safeInvoke = isTauriContext() ? invoke : mockInvoke;

export const systemService = {
  async getSystemInfo(): Promise<SystemInfo> {
    return await safeInvoke<SystemInfo>('get_system_info');
  },
};

export const mlService = {
  async listAvailableModels(): Promise<MLModel[]> {
    return await safeInvoke<MLModel[]>('list_available_models');
  },

  async loadModel(modelId: string): Promise<boolean> {
    return await safeInvoke<boolean>('load_model', { modelId });
  },
};

export const networkService = {
  async discoverNetworkNodes(): Promise<NetworkNode[]> {
    return await safeInvoke<NetworkNode[]>('discover_network_nodes');
  },

  async connectToFlowerServer(serverAddress: string): Promise<boolean> {
    return await safeInvoke<boolean>('connect_to_flower_server', { serverAddress });
  },
};

// New State Management Services
export const stateService = {
  async getSetupStatus(): Promise<SetupStatus> {
    return await safeInvoke<SetupStatus>('get_setup_status');
  },

  async getHardwareProfile(): Promise<HardwareProfile | null> {
    return await safeInvoke<HardwareProfile | null>('get_hardware_profile');
  },

  async getAuthState(): Promise<AuthState> {
    return await safeInvoke<AuthState>('get_auth_state');
  },

  async getTrainingState(): Promise<TrainingState> {
    return await safeInvoke<TrainingState>('get_training_state');
  },

  async getAppConfig(): Promise<AppConfig> {
    return await safeInvoke<AppConfig>('get_app_config');
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
