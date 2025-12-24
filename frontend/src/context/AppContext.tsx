import React, { createContext, useContext, useState, ReactNode } from 'react';
import type { ModelFile } from '../types/mesh';
import type { CrochetPattern } from '../types/pattern';
import type { CrochetConfig } from '../types/config';
import { DEFAULT_CONFIG } from '../types/config';

interface AppState {
  modelFile: ModelFile | null;
  config: CrochetConfig;
  pattern: CrochetPattern | null;
  isGenerating: boolean;
}

interface AppContextType extends AppState {
  setModelFile: (file: ModelFile | null) => void;
  setConfig: (config: CrochetConfig) => void;
  setPattern: (pattern: CrochetPattern | null) => void;
  setIsGenerating: (isGenerating: boolean) => void;
  resetApp: () => void;
}

const AppContext = createContext<AppContextType | undefined>(undefined);

interface AppProviderProps {
  children: ReactNode;
}

export const AppProvider: React.FC<AppProviderProps> = ({ children }) => {
  const [modelFile, setModelFile] = useState<ModelFile | null>(null);
  const [config, setConfig] = useState<CrochetConfig>(DEFAULT_CONFIG);
  const [pattern, setPattern] = useState<CrochetPattern | null>(null);
  const [isGenerating, setIsGenerating] = useState(false);

  const resetApp = () => {
    setModelFile(null);
    setPattern(null);
    setIsGenerating(false);
  };

  const value: AppContextType = {
    modelFile,
    config,
    pattern,
    isGenerating,
    setModelFile,
    setConfig,
    setPattern,
    setIsGenerating,
    resetApp,
  };

  return <AppContext.Provider value={value}>{children}</AppContext.Provider>;
};

export const useApp = (): AppContextType => {
  const context = useContext(AppContext);
  if (context === undefined) {
    throw new Error('useApp must be used within an AppProvider');
  }
  return context;
};
