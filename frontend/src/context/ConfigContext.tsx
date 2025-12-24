import React, { createContext, useContext, useState, ReactNode, useEffect } from 'react';
import type { CrochetConfig } from '../types/config';
import { DEFAULT_CONFIG } from '../types/config';
import { STORAGE_KEYS } from '../utils/constants';

interface ConfigContextType {
  config: CrochetConfig;
  updateConfig: (updates: Partial<CrochetConfig>) => void;
  resetConfig: () => void;
}

const ConfigContext = createContext<ConfigContextType | undefined>(undefined);

interface ConfigProviderProps {
  children: ReactNode;
}

export const ConfigProvider: React.FC<ConfigProviderProps> = ({ children }) => {
  const [config, setConfig] = useState<CrochetConfig>(() => {
    // Load config from localStorage if available
    try {
      const saved = localStorage.getItem(STORAGE_KEYS.CONFIG);
      if (saved) {
        return JSON.parse(saved);
      }
    } catch (error) {
      console.error('Failed to load config from localStorage:', error);
    }
    return DEFAULT_CONFIG;
  });

  // Save config to localStorage whenever it changes
  useEffect(() => {
    try {
      localStorage.setItem(STORAGE_KEYS.CONFIG, JSON.stringify(config));
    } catch (error) {
      console.error('Failed to save config to localStorage:', error);
    }
  }, [config]);

  const updateConfig = (updates: Partial<CrochetConfig>) => {
    setConfig((prev) => ({
      ...prev,
      ...updates,
      gauge: updates.gauge ? { ...prev.gauge, ...updates.gauge } : prev.gauge,
      yarn: updates.yarn ? { ...prev.yarn, ...updates.yarn } : prev.yarn,
      construction: updates.construction
        ? { ...prev.construction, ...updates.construction }
        : prev.construction,
      optimization: updates.optimization
        ? { ...prev.optimization, ...updates.optimization }
        : prev.optimization,
    }));
  };

  const resetConfig = () => {
    setConfig(DEFAULT_CONFIG);
    try {
      localStorage.removeItem(STORAGE_KEYS.CONFIG);
    } catch (error) {
      console.error('Failed to remove config from localStorage:', error);
    }
  };

  const value: ConfigContextType = {
    config,
    updateConfig,
    resetConfig,
  };

  return <ConfigContext.Provider value={value}>{children}</ConfigContext.Provider>;
};

export const useConfig = (): ConfigContextType => {
  const context = useContext(ConfigContext);
  if (context === undefined) {
    throw new Error('useConfig must be used within a ConfigProvider');
  }
  return context;
};
