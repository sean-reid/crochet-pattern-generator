import { useState } from 'react';
import { Pencil, Settings, Eye, Download } from 'lucide-react';
import DrawingCanvas from './components/DrawingCanvas';
import ConfigurationPanel from './components/ConfigurationPanel';
import PatternPreview from './components/PatternPreview';
import ExportPanel from './components/ExportPanel';
import type { ProfileCurve, AmigurumiConfig, CrochetPattern } from './types';

type Tab = 'draw' | 'configure' | 'preview' | 'export';

function App() {
  const [currentTab, setCurrentTab] = useState<Tab>('draw');
  const [profile, setProfile] = useState<ProfileCurve | null>(null);
  const [config, setConfig] = useState<AmigurumiConfig>({
    total_height_cm: 10,
    yarn: {
      gauge_stitches_per_cm: 3.0,
      gauge_rows_per_cm: 3.0,
      recommended_hook_size_mm: 3.5,
    },
  });
  const [pattern, setPattern] = useState<CrochetPattern | null>(null);
  const [error, setError] = useState<string | null>(null);

  const tabs = [
    { id: 'draw' as Tab, label: 'Draw', icon: Pencil, disabled: false },
    { id: 'configure' as Tab, label: 'Configure', icon: Settings, disabled: !profile },
    { id: 'preview' as Tab, label: 'Preview', icon: Eye, disabled: !pattern },
    { id: 'export' as Tab, label: 'Export', icon: Download, disabled: !pattern },
  ];

  const handleProfileChange = (newProfile: ProfileCurve) => {
    setProfile(newProfile);
  };

  const handleConfigChange = (newConfig: AmigurumiConfig) => {
    setConfig(newConfig);
  };

  const handlePatternGenerated = (newPattern: CrochetPattern) => {
    setPattern(newPattern);
    setError(null);
    setCurrentTab('preview');
  };

  const handleError = (errorMessage: string) => {
    setError(errorMessage);
  };

  return (
    <div className="min-h-screen bg-cream-100">
      {/* Header */}
      <header className="bg-white border-b border-slate-200">
        <div className="max-w-7xl mx-auto px-6 py-6">
          <h1 className="text-3xl font-bold text-slate-900">
            Crochet Pattern Generator
          </h1>
          <p className="text-slate-600 mt-2">
            Create custom amigurumi patterns from your drawings
          </p>
        </div>
      </header>

      {/* Navigation Tabs */}
      <nav className="bg-white border-b border-slate-200">
        <div className="max-w-7xl mx-auto px-6">
          <div className="flex space-x-1">
            {tabs.map((tab) => {
              const Icon = tab.icon;
              const isActive = currentTab === tab.id;
              const isDisabled = tab.disabled;
              return (
                <button
                  key={tab.id}
                  onClick={() => !isDisabled && setCurrentTab(tab.id)}
                  disabled={isDisabled}
                  className={`
                    flex items-center gap-2 px-6 py-4 font-medium border-b-2 transition-colors
                    ${
                      isActive
                        ? 'border-terracotta-500 text-terracotta-500'
                        : isDisabled
                        ? 'border-transparent text-slate-300 cursor-not-allowed'
                        : 'border-transparent text-slate-600 hover:text-slate-900 cursor-pointer'
                    }
                  `}
                >
                  <Icon size={20} />
                  <span>{tab.label}</span>
                </button>
              );
            })}
          </div>
        </div>
      </nav>

      {/* Error Display */}
      {error && (
        <div className="max-w-7xl mx-auto px-6 py-4">
          <div className="bg-clay-500 text-white px-6 py-4 rounded-xl">
            <p className="font-medium">Error</p>
            <p className="mt-1">{error}</p>
          </div>
        </div>
      )}

      {/* Main Content */}
      <main className="max-w-7xl mx-auto px-6 py-8">
        {currentTab === 'draw' && (
          <DrawingCanvas
            profile={profile}
	    config={config}
            onChange={handleProfileChange}
            onError={handleError}
          />
        )}

        {currentTab === 'configure' && (
          <ConfigurationPanel
            config={config}
            profile={profile}
            onChange={handleConfigChange}
            onGeneratePattern={handlePatternGenerated}
            onError={handleError}
          />
        )}

        {currentTab === 'preview' && (
          <PatternPreview pattern={pattern} config={config} />
        )}

        {currentTab === 'export' && (
          <ExportPanel pattern={pattern} config={config} />
        )}
      </main>
    </div>
  );
}

export default App;
