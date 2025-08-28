import React from 'react';
import './TauriContextProvider.css';

interface TauriContextProviderProps {
  children: React.ReactNode;
}

const TauriContextProvider: React.FC<TauriContextProviderProps> = ({ children }) => {
  const isTauriContext = typeof window !== 'undefined' && (window as any).__TAURI__ !== undefined;

  if (!isTauriContext) {
    return (
      <div className="tauri-context-error">
        <div className="error-container">
          <h1>⚠️ Tauri Context Required</h1>
          <p>This application must be run through the Tauri desktop application, not in a web browser.</p>
          
          <div className="instructions">
            <h2>To run the application correctly:</h2>
            <ol>
              <li>Open a terminal in the project directory</li>
              <li>Run: <code>npm run tauri dev</code></li>
              <li>The desktop application window should open automatically</li>
            </ol>
          </div>

          <div className="current-context">
            <h3>Current Context:</h3>
            <p>You are accessing: <strong>http://localhost:1420</strong> (Vite dev server)</p>
            <p>You should use: <strong>Desktop Application Window</strong></p>
          </div>

          <div className="note">
            <p><strong>Note:</strong> OAuth2 and other Tauri features will not work in a regular browser.</p>
          </div>
        </div>
      </div>
    );
  }

  return <>{children}</>;
};

export default TauriContextProvider;
