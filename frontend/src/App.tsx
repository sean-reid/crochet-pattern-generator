import React, { useState, useEffect } from 'react';
import { AppProvider } from './context/AppContext';
import { ConfigProvider } from './context/ConfigContext';
import { useWasmWorker } from './hooks/useWasmWorker';
import FileUploadZone from './components/FileUploadZone';
import ModelViewer from './components/ModelViewer';
import ConfigPanel from './components/ConfigPanel';
import PatternPreview from './components/PatternPreview';
import ExportPanel from './components/ExportPanel';
import { Icon } from './components/common/Icon';
import { Spinner } from './components/common/Loading';
import './App.css';

const AppContent: React.FC = () => {
  const [sidebarOpen, setSidebarOpen] = useState(true);
  const { initializeWasm, isInitializing, isInitialized, initError } = useWasmWorker();

  // Initialize WASM module on app mount
  useEffect(() => {
    initializeWasm();
  }, [initializeWasm]);

  // Show loading screen while WASM initializes
  if (isInitializing) {
    return (
      <div className="app-loading">
        <Spinner size="large" />
        <p style={{ marginTop: 'var(--spacing-16)', color: 'var(--color-gray-medium)' }}>
          Loading WASM module...
        </p>
      </div>
    );
  }

  // Show error if WASM failed to load
  if (initError) {
    return (
      <div className="app-loading">
        <Icon name="AlertCircle" size={48} color="var(--color-burgundy)" />
        <p style={{ marginTop: 'var(--spacing-16)', color: 'var(--color-burgundy)', fontWeight: 'var(--font-weight-semibold)' }}>
          Failed to load WASM module
        </p>
        <p style={{ marginTop: 'var(--spacing-8)', color: 'var(--color-gray-medium)', fontSize: 'var(--font-size-sm)', maxWidth: '400px', textAlign: 'center' }}>
          {initError}
        </p>
        <p style={{ marginTop: 'var(--spacing-16)', color: 'var(--color-gray-medium)', fontSize: 'var(--font-size-sm)' }}>
          Make sure the WASM module is built and placed in <code>public/wasm/</code>
        </p>
      </div>
    );
  }

  // Show warning if WASM not initialized (shouldn't happen but safety check)
  if (!isInitialized) {
    return (
      <div className="app-loading">
        <Icon name="AlertTriangle" size={48} color="var(--color-amber-soft)" />
        <p style={{ marginTop: 'var(--spacing-16)', color: 'var(--color-gray-medium)' }}>
          WASM module not initialized
        </p>
      </div>
    );
  }

  return (
    <div className="app">
          <header className="app-header">
            <div className="header-content">
              <div className="header-left">
                <Icon name="Scissors" size={28} color="var(--color-terracotta)" />
                <h1 className="app-title">Crochet Pattern Generator</h1>
              </div>
              <button
                className="menu-button"
                onClick={() => setSidebarOpen(!sidebarOpen)}
                aria-label={sidebarOpen ? 'Close sidebar' : 'Open sidebar'}
              >
                <Icon name={sidebarOpen ? 'X' : 'Menu'} size={24} />
              </button>
            </div>
          </header>

          <div className="app-layout">
            {/* Left Sidebar - Configuration */}
            <aside className={`sidebar ${sidebarOpen ? 'sidebar-open' : ''}`}>
              <div className="sidebar-content">
                <section className="sidebar-section">
                  <h2 className="sidebar-heading">Upload Model</h2>
                  <FileUploadZone />
                </section>

                <section className="sidebar-section">
                  <h2 className="sidebar-heading">Configuration</h2>
                  <ConfigPanel />
                </section>
              </div>
            </aside>

            {/* Main Content Area */}
            <main className="main-content">
              <div className="viewer-section">
                <ModelViewer />
              </div>
            </main>

            {/* Right Panel - Pattern Output */}
            <aside className="right-panel">
              <div className="panel-content">
                <section className="panel-section">
                  <h2 className="panel-heading">Pattern Preview</h2>
                  <PatternPreview />
                </section>

                <section className="panel-section">
                  <h2 className="panel-heading">Export</h2>
                  <ExportPanel />
                </section>
              </div>
            </aside>
          </div>
        </div>
      );
};

const App: React.FC = () => {
  return (
    <AppProvider>
      <ConfigProvider>
        <AppContent />
      </ConfigProvider>
    </AppProvider>
  );
};

export default App;
