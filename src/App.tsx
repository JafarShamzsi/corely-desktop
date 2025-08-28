import React from 'react';
import { AuthProvider } from '@/hooks/useAuth';
import AuthWrapper from '@/components/AuthWrapper';
import TauriContextProvider from '@/components/TauriContextProvider';
import './App.css';

const App: React.FC = () => {
  return (
    <TauriContextProvider>
      <AuthProvider>
        <div className="app">
          <AuthWrapper />
        </div>
      </AuthProvider>
    </TauriContextProvider>
  );
};

export default App;
