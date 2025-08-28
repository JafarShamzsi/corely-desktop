import React, { createContext, useContext, useEffect, useState, ReactNode } from 'react';
import { authService, type User, type SessionInfo } from '@/services/authService';
import { open } from '@tauri-apps/plugin-shell';

interface AuthContextType {
  // State
  user: User | null;
  session: SessionInfo | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  error: string | null;

  // Actions
  login: (username: string, password: string) => Promise<boolean>;
  register: (username: string, email: string, password: string) => Promise<boolean>;
  logout: () => Promise<void>;
  loginWithGoogle: () => Promise<void>;
  loginWithGitHub: () => Promise<void>;
  handleOAuth2Callback: (code: string, state: string) => Promise<void>;
  clearError: () => void;
  refreshSession: () => Promise<void>;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

interface AuthProviderProps {
  children: ReactNode;
}

export const AuthProvider: React.FC<AuthProviderProps> = ({ children }) => {
  const [user, setUser] = useState<User | null>(null);
  const [session, setSession] = useState<SessionInfo | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const isAuthenticated = user !== null && session !== null;

  // Initialize authentication state
  useEffect(() => {
    initializeAuth();
  }, []);

  const initializeAuth = async () => {
    try {
      setIsLoading(true);
      const currentSession = await authService.getCurrentSession();
      
      if (currentSession) {
        const isValid = await authService.validateToken();
        if (isValid) {
          setSession(currentSession);
          setUser(currentSession.user);
        } else {
          // Token expired, try to refresh
          try {
            await refreshSession();
          } catch {
            // Refresh failed, clear session
            await logout();
          }
        }
      }
    } catch (err) {
      console.error('Failed to initialize auth:', err);
      setError('Failed to initialize authentication');
    } finally {
      setIsLoading(false);
    }
  };

  const login = async (username: string, password: string): Promise<boolean> => {
    try {
      setIsLoading(true);
      setError(null);

      const response = await authService.login({ username, password });
      
      if (response.success && response.user) {
        setUser(response.user);
        // Get the current session after login
        const currentSession = await authService.getCurrentSession();
        if (currentSession) {
          setSession(currentSession);
        }
        return true;
      } else {
        setError(response.message || 'Login failed');
        return false;
      }
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Login failed';
      setError(errorMessage);
      return false;
    } finally {
      setIsLoading(false);
    }
  };

  const register = async (username: string, email: string, password: string): Promise<boolean> => {
    try {
      setIsLoading(true);
      setError(null);

      const response = await authService.register({ username, email, password });
      
      if (response.success && response.user) {
        setUser(response.user);
        // Get the current session after registration
        const currentSession = await authService.getCurrentSession();
        if (currentSession) {
          setSession(currentSession);
        }
        return true;
      } else {
        setError(response.message || 'Registration failed');
        return false;
      }
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Registration failed';
      setError(errorMessage);
      return false;
    } finally {
      setIsLoading(false);
    }
  };

  const logout = async (): Promise<void> => {
    try {
      setIsLoading(true);
      await authService.logout();
    } catch (err) {
      console.error('Logout error:', err);
    } finally {
      setUser(null);
      setSession(null);
      setError(null);
      setIsLoading(false);
    }
  };

  const loginWithGoogle = async (): Promise<void> => {
    try {
      setIsLoading(true);
      setError(null);

      const oauth2Response = await authService.startGoogleOAuth2();
      
      // Store PKCE verifier and state for callback
      sessionStorage.setItem('oauth2_pkce_verifier', oauth2Response.pkce_verifier);
      sessionStorage.setItem('oauth2_state', oauth2Response.csrf_token);
      
      // Open OAuth2 authorization URL in system browser using Tauri shell
      try {
        await open(oauth2Response.auth_url);
        setError('Google OAuth2 flow initiated. Please complete authorization in your browser and return to this app.');
      } catch (openError) {
        // Fallback to window.open if Tauri shell fails
        window.open(oauth2Response.auth_url, '_blank');
        setError('Google OAuth2 flow initiated. Please complete authorization in the opened browser tab.');
      }
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Google OAuth2 failed';
      setError(errorMessage);
    } finally {
      setIsLoading(false);
    }
  };

  const loginWithGitHub = async (): Promise<void> => {
    try {
      setIsLoading(true);
      setError(null);

      const oauth2Response = await authService.startGitHubOAuth2();
      
      // Store PKCE verifier and state for callback
      sessionStorage.setItem('oauth2_pkce_verifier', oauth2Response.pkce_verifier);
      sessionStorage.setItem('oauth2_state', oauth2Response.csrf_token);
      
      // Open OAuth2 authorization URL in system browser using Tauri shell
      try {
        await open(oauth2Response.auth_url);
        setError('GitHub OAuth2 flow initiated. Please complete authorization in your browser and return to this app.');
      } catch (openError) {
        // Fallback to window.open if Tauri shell fails
        window.open(oauth2Response.auth_url, '_blank');
        setError('GitHub OAuth2 flow initiated. Please complete authorization in the opened browser tab.');
      }
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'GitHub OAuth2 failed';
      setError(errorMessage);
    } finally {
      setIsLoading(false);
    }
  };

  const refreshSession = async (): Promise<void> => {
    try {
      const response = await authService.refreshAccessToken();
      if (response.success && response.user) {
        setUser(response.user);
        const currentSession = await authService.getCurrentSession();
        if (currentSession) {
          setSession(currentSession);
        }
      } else {
        throw new Error('Token refresh failed');
      }
    } catch (err) {
      console.error('Session refresh failed:', err);
      await logout();
      throw err;
    }
  };

  const handleOAuth2Callback = async (code: string, state: string): Promise<void> => {
    try {
      setIsLoading(true);
      setError(null);
      
      // Retrieve and validate the stored state and PKCE verifier
      const storedState = sessionStorage.getItem('oauth2_state');
      const pkceVerifier = sessionStorage.getItem('oauth2_pkce_verifier');
      
      if (!pkceVerifier) {
        throw new Error('PKCE verifier not found. Please restart the OAuth2 flow.');
      }
      
      if (!storedState || storedState !== state) {
        throw new Error('Invalid state parameter. Possible CSRF attack.');
      }
      
      const response = await authService.handleOAuth2Callback(code, pkceVerifier);
      
      if (response.success && response.user) {
        setUser(response.user);
        const currentSession = await authService.getCurrentSession();
        if (currentSession) {
          setSession(currentSession);
        }
        
        // Clean up session storage
        sessionStorage.removeItem('oauth2_pkce_verifier');
        sessionStorage.removeItem('oauth2_state');
      } else {
        throw new Error(response.message || 'OAuth2 authentication failed');
      }
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'OAuth2 callback failed';
      setError(errorMessage);
      throw err;
    } finally {
      setIsLoading(false);
    }
  };

  const clearError = (): void => {
    setError(null);
  };

  // Auto-refresh token before expiration
  useEffect(() => {
    if (!session) return;

    const expiresAt = new Date(session.expires_at).getTime();
    const now = Date.now();
    const timeUntilExpiry = expiresAt - now;
    
    // Refresh 5 minutes before expiration
    const refreshTime = Math.max(timeUntilExpiry - 5 * 60 * 1000, 60 * 1000);

    const refreshTimer = setTimeout(() => {
      refreshSession().catch(console.error);
    }, refreshTime);

    return () => clearTimeout(refreshTimer);
  }, [session]);

  const value: AuthContextType = {
    user,
    session,
    isAuthenticated,
    isLoading,
    error,
    login,
    register,
    logout,
    loginWithGoogle,
    loginWithGitHub,
    handleOAuth2Callback,
    clearError,
    refreshSession,
  };

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
};

export const useAuth = (): AuthContextType => {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
};
