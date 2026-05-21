import React, { useState } from 'react';
import Dashboard from './components/Dashboard';
import ProviderManager from './components/ProviderManager';
import RequestInspector from './components/RequestInspector';
import MetricsView from './components/MetricsView';
import SettingsView from './components/SettingsView';
import { Toaster } from 'react-hot-toast';

type Tab = 'dashboard' | 'providers' | 'requests' | 'metrics' | 'settings';

const tabs: { id: Tab; label: string; icon: string }[] = [
  { id: 'dashboard', label: 'Dashboard', icon: '📊' },
  { id: 'providers', label: 'Providers', icon: '🔗' },
  { id: 'requests', label: 'Requests', icon: '📨' },
  { id: 'metrics', label: 'Metrics', icon: '📈' },
  { id: 'settings', label: 'Settings', icon: '⚙️' },
];

function App() {
  const [activeTab, setActiveTab] = useState<Tab>('dashboard');
  const [isDark, setIsDark] = useState(true);

  const renderContent = () => {
    switch (activeTab) {
      case 'dashboard':
        return <Dashboard />;
      case 'providers':
        return <ProviderManager />;
      case 'requests':
        return <RequestInspector />;
      case 'metrics':
        return <MetricsView />;
      case 'settings':
        return <SettingsView onThemeToggle={() => setIsDark(!isDark)} isDark={isDark} />;
      default:
        return <Dashboard />;
    }
  };

  return (
    <div className={`min-h-screen ${isDark ? 'bg-gray-900' : 'bg-gray-50'}`}>
      <nav className={`${isDark ? 'bg-gray-800 border-gray-700' : 'bg-white border-gray-200'} border-b`}>
        <div className="max-w-7xl mx-auto px-4">
          <div className="flex items-center justify-between h-16">
            <div className="flex items-center space-x-8">
              <h1 className={`text-xl font-bold ${isDark ? 'text-white' : 'text-gray-900'}`}>
                🔗 ModelLink
              </h1>
              <div className="flex space-x-1">
                {tabs.map((tab) => (
                  <button
                    key={tab.id}
                    onClick={() => setActiveTab(tab.id)}
                    className={`px-4 py-2 rounded-lg transition-colors ${
                      activeTab === tab.id
                        ? 'bg-blue-600 text-white'
                        : isDark
                        ? 'text-gray-300 hover:bg-gray-700'
                        : 'text-gray-600 hover:bg-gray-100'
                    }`}
                  >
                    <span className="mr-2">{tab.icon}</span>
                    {tab.label}
                  </button>
                ))}
              </div>
            </div>
            <div className="flex items-center space-x-4">
              <div className={`text-sm ${isDark ? 'text-gray-400' : 'text-gray-500'}`}>
                v0.1.0
              </div>
              <button
                onClick={() => setIsDark(!isDark)}
                className={`p-2 rounded-lg ${
                  isDark ? 'bg-gray-700 text-yellow-400' : 'bg-gray-200 text-gray-700'
                }`}
              >
                {isDark ? '☀️' : '🌙'}
              </button>
            </div>
          </div>
        </div>
      </nav>

      <main className="max-w-7xl mx-auto px-4 py-6">
        {renderContent()}
      </main>

      <Toaster position="bottom-right" />
    </div>
  );
}

export default App;
