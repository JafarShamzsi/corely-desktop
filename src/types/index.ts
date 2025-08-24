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
