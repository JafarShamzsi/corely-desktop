import React, { useEffect } from 'react';
import { useAuth } from '../hooks/useAuth';

interface AuthCallbackProps {
  onComplete: () => void;
  onError: (error: string) => void;
}

const AuthCallback: React.FC<AuthCallbackProps> = ({ onComplete, onError }) => {
  const { handleOAuth2Callback } = useAuth();

  useEffect(() => {
    const handleCallback = async () => {
      // Get URL parameters
      const urlParams = new URLSearchParams(window.location.search);
      const code = urlParams.get('code');
      const state = urlParams.get('state');
      
      if (code && state) {
        try {
          await handleOAuth2Callback(code, state);
          onComplete();
        } catch (error) {
          console.error('OAuth2 callback error:', error);
          onError(error instanceof Error ? error.message : 'OAuth2 authentication failed');
        }
      } else {
        onError('Invalid OAuth2 callback parameters');
      }
    };

    handleCallback();
  }, [handleOAuth2Callback, onComplete, onError]);

  return (
    <div className="auth-callback">
      <div className="processing">
        <div className="spinner"></div>
        <p>Processing authentication...</p>
      </div>
    </div>
  );
};

export default AuthCallback;
