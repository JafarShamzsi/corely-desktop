import React, { useState } from 'react';
import { useAuth } from '@/hooks/useAuth';
import LoginForm from '@/components/LoginForm';
import RegisterForm from '@/components/RegisterForm';
import MainDashboard from '@/components/MainDashboard';

const AuthWrapper: React.FC = () => {
  const { isAuthenticated, isLoading } = useAuth();
  const [showRegister, setShowRegister] = useState(false);

  // Show loading screen while checking authentication
  if (isLoading) {
    return (
      <div className="auth-loading">
        <div className="loading-container">
          <div className="loading-spinner"></div>
          <h2>AI Training Marketplace</h2>
          <p>Initializing application...</p>
        </div>
      </div>
    );
  }

  // Show dashboard if authenticated
  if (isAuthenticated) {
    return <MainDashboard />;
  }

  // Show authentication forms if not authenticated
  return (
    <>
      {showRegister ? (
        <RegisterForm onSwitchToLogin={() => setShowRegister(false)} />
      ) : (
        <LoginForm onSwitchToRegister={() => setShowRegister(true)} />
      )}
    </>
  );
};

export default AuthWrapper;
