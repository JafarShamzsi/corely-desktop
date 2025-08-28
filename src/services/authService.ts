import { invoke } from '@tauri-apps/api/core';

// Check if we're running in Tauri context
const isTauriContext = () => {
  return typeof window !== 'undefined' && (window as any).__TAURI__ !== undefined;
};

// Mock invoke function for development in browser
const mockInvoke = async (command: string, args?: any) => {
  console.warn(`Mock Tauri invoke called: ${command}`, args);
  throw new Error('Tauri context not available. Please run the app through Tauri desktop application.');
};

// Safe invoke that works in both Tauri and browser contexts
const safeInvoke = isTauriContext() ? invoke : mockInvoke;

// Authentication types
export interface User {
  id: string;
  username: string;
  email?: string;
  oauth_provider?: 'google' | 'github';
  created_at: string;
  last_login: string;
}

export interface LoginRequest {
  username: string;
  password: string;
}

export interface RegisterRequest {
  username: string;
  email: string;
  password: string;
}

export interface AuthResponse {
  success: boolean;
  user?: User;
  access_token?: string;
  refresh_token?: string;
  expires_in?: number;
  message?: string;
}

export interface SessionInfo {
  user: User;
  session_id: string;
  expires_at: string;
  device_fingerprint: string;
}

export interface OAuth2Response {
  auth_url: string;
  csrf_token: string;
  pkce_verifier: string;
}

// Authentication Service
export const authService = {
  // Username/Password Authentication
  async login(credentials: LoginRequest): Promise<AuthResponse> {
    return await safeInvoke<AuthResponse>('login_user', {
      username: credentials.username,
      password: credentials.password
    });
  },

  async register(userData: RegisterRequest): Promise<AuthResponse> {
    return await safeInvoke<AuthResponse>('register_user', {
      username: userData.username,
      email: userData.email,
      password: userData.password
    });
  },

  async logout(): Promise<void> {
    return await safeInvoke<void>('secure_logout_user');
  },

  // OAuth2 Authentication
  async startGoogleOAuth2(): Promise<OAuth2Response> {
    return await safeInvoke<OAuth2Response>('get_oauth2_auth_url', { provider: 'Google' });
  },

  async startGitHubOAuth2(): Promise<OAuth2Response> {
    return await safeInvoke<OAuth2Response>('get_oauth2_auth_url', { provider: 'GitHub' });
  },

  async handleOAuth2Callback(code: string, codeVerifier: string): Promise<AuthResponse> {
    return await safeInvoke<AuthResponse>('oauth2_login', { 
      provider: 'Google', 
      authorization_code: code, 
      code_verifier: codeVerifier 
    });
  },

  // Session Management
  async getCurrentSession(): Promise<SessionInfo | null> {
    try {
      return await safeInvoke<SessionInfo>('get_current_session');
    } catch {
      return null;
    }
  },

  async validateToken(): Promise<boolean> {
    try {
      return await safeInvoke<boolean>('validate_token');
    } catch {
      return false;
    }
  },

  async refreshAccessToken(): Promise<AuthResponse> {
    return await safeInvoke<AuthResponse>('refresh_access_token');
  },

  // Device and Security
  async getDeviceFingerprint(): Promise<string> {
    return await safeInvoke<string>('get_device_fingerprint');
  },

  async getActiveSessions(): Promise<SessionInfo[]> {
    return await safeInvoke<SessionInfo[]>('get_active_sessions');
  },

  async revokeSession(sessionId: string): Promise<void> {
    return await safeInvoke<void>('revoke_session', { sessionId });
  },

  // Authentication Status
  async isAuthenticated(): Promise<boolean> {
    try {
      const session = await this.getCurrentSession();
      return session !== null;
    } catch {
      return false;
    }
  }
};
