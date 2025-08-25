// Core types for the AI Training Desktop application
export interface SystemInfo {
  cpu_count: number;
  total_memory: number;
  available_memory: number;
  os_type: string;
  gpu_info: string[];
}

export interface MLModel {
  id: string;
  name: string;
  framework: string;
  size_mb: number;
  is_loaded: boolean;
}

export interface NetworkNode {
  id: string;
  address: string;
  status: string;
  last_seen?: string;
}

export interface TrainingJob {
  id: string;
  model_id: string;
  status: 'pending' | 'running' | 'completed' | 'failed';
  progress: number;
  created_at: string;
  updated_at: string;
}

// State Management Types
export interface CpuInfo {
  name: string;
  cores: number;
  threads: number;
  base_frequency: number;
  max_frequency: number;
  cache_size: number;
  architecture: string;
  supports_avx: boolean;
  supports_avx2: boolean;
}

export interface GpuInfo {
  name: string;
  vendor: string;
  memory: number;
  compute_capability?: string;
  cuda_cores?: number;
  driver_version: string;
  supports_ml: boolean;
}

export interface MemoryInfo {
  total: number;
  available: number;
  speed?: number;
  memory_type: string;
}

export interface StorageInfo {
  total_space: number;
  available_space: number;
  storage_type: string;
  read_speed?: number;
  write_speed?: number;
}

export interface HardwareProfile {
  cpu: CpuInfo;
  gpu?: GpuInfo;
  memory: MemoryInfo;
  storage: StorageInfo;
  last_updated: string;
}

export type SetupStatus = 
  | { 
      type: 'SystemAnalysis';
      progress: number;
      details: string;
      started_at: string;
    }
  | {
      type: 'HardwareDetection';
      progress: number;
      cpu_detected: boolean;
      gpu_detected: boolean;
      memory_detected: boolean;
      started_at: string;
    }
  | {
      type: 'DriverCheck';
      progress: number;
      drivers_status: Record<string, boolean>;
      recommendations: string[];
      started_at: string;
    }
  | {
      type: 'EnvironmentSetup';
      progress: number;
      dependencies_installed: string[];
      environment_ready: boolean;
      started_at: string;
    }
  | {
      type: 'Complete';
      completed_at: string;
      summary: string;
    }
  | {
      type: 'Failed';
      error: string;
      failed_at: string;
      retry_count: number;
    };

export interface AuthState {
  is_authenticated: boolean;
  user_id?: string;
  username?: string;
  email?: string;
  token?: string;
  token_expires_at?: string;
  permissions: string[];
  last_login?: string;
}

export interface TrainingState {
  is_training: boolean;
  current_session_id?: string;
  current_model?: string;
  progress: number;
  epoch: number;
  batch: number;
  loss?: number;
  accuracy?: number;
  training_start?: string;
  estimated_completion?: string;
}

export interface AppConfig {
  user_id?: string;
  api_endpoint: string;
  enable_distributed_training: boolean;
  max_memory_usage_gb: number;
  preferred_gpu?: string;
  auto_save_interval: number;
  log_level: string;
  enable_telemetry: boolean;
  data_directory: string;
  temp_directory: string;
}

export type AppEvent = 
  | {
      type: 'SetupStatusChanged';
      old_status: SetupStatus;
      new_status: SetupStatus;
      timestamp: string;
    }
  | {
      type: 'HardwareProfileUpdated';
      profile: HardwareProfile;
      timestamp: string;
    }
  | {
      type: 'AuthStateChanged';
      authenticated: boolean;
      user_id?: string;
      timestamp: string;
    }
  | {
      type: 'TrainingSessionChanged';
      session_id?: string;
      model_name?: string;
      status: string;
      timestamp: string;
    }
  | {
      type: 'ResourceUsageUpdated';
      cpu_usage: number;
      memory_usage: number;
      gpu_usage?: number;
      timestamp: string;
    }
  | {
      type: 'ErrorOccurred';
      error: string;
      context: string;
      timestamp: string;
    };
