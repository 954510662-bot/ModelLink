import React, { useState, useEffect } from 'react';
import Dashboard from './components/Dashboard';
import ProviderManager from './components/ProviderManager';
import RequestInspector from './components/RequestInspector';
import MetricsView from './components/MetricsView';
import SettingsView from './components/SettingsView';
import AlertsView from './components/AlertsView';
import Onboarding from './components/Onboarding';
import { Toaster } from 'react-hot-toast';

type Tab = 'dashboard' | 'providers' | 'requests' | 'metrics' | 'alerts' | 'settings';

const tabs: { id: Tab; label: string; icon: string }[] = [
  { id: 'dashboard', label: 'Dashboard', icon: '📊' },
  { id: 'providers', label: 'Providers', icon: '🔗' },
  { id: 'requests', label: 'Requests', icon: '📨' },
  { id: 'metrics', label: 'Metrics', icon: '📈' },
  { id: 'alerts', label: 'Alerts', icon: '🔔' },
  { id: 'settings', label: 'Settings', icon: '⚙️' },
];

function App() {
  const [activeTab, setActiveTab] = useState<Tab>('dashboard');
  const [isDark, setIsDark] = useState(true);
  const [isLoading, setIsLoading] = useState(true);
  const [alertCount, setAlertCount] = useState(0);

  useEffect(() => {
    const savedTheme = localStorage.getItem('modelLink-theme');
    if (savedTheme) {
      setIsDark(savedTheme === 'dark');
    }

    const timer = setTimeout(() => {
      setIsLoading(false);
    }, 800);

    fetchAlertCount();

    const interval = setInterval(fetchAlertCount, 30000);

    return () => {
      clearTimeout(timer);
      clearInterval(interval);
    };
  }, []);

  const fetchAlertCount = async () => {
    try {
      const res = await fetch('http://localhost:9191/alerts/stats');
      if (res.ok) {
        const data = await res.json();
        setAlertCount(data.active_alerts || 0);
      }
    } catch (error) {
      console.error('Failed to fetch alert count:', error);
    }
  };

  const handleThemeToggle = () => {
    const newTheme = !isDark;
    setIsDark(newTheme);
    localStorage.setItem('modelLink-theme', newTheme ? 'dark' : 'light');
  };

  const renderContent = () => {
    if (isLoading) {
      return <LoadingSkeleton isDark={isDark} />;
    }

    switch (activeTab) {
      case 'dashboard':
        return <Dashboard />;
      case 'providers':
        return <ProviderManager />;
      case 'requests':
        return <RequestInspector />;
      case 'metrics':
        return <MetricsView />;
      case 'alerts':
        return <AlertsView />;
      case 'settings':
        return <SettingsView onThemeToggle={handleThemeToggle} isDark={isDark} />;
      default:
        return <Dashboard />;
    }
  };

  return (
    <div 
      className={`min-h-screen transition-colors duration-300 ${
        isDark ? 'bg-gray-900' : 'bg-gray-50'
      }`}
    >
      <nav 
        className={`${
          isDark ? 'bg-gray-800 border-gray-700' : 'bg-white border-gray-200'
        } border-b transition-colors duration-300`}
      >
        <div className="max-w-7xl mx-auto px-4">
          <div className="flex items-center justify-between h-16">
            <div className="flex items-center space-x-8">
              <h1 
                className={`text-xl font-bold transition-colors duration-300 ${
                  isDark ? 'text-white' : 'text-gray-900'
                }`}
              >
                🔗 ModelLink
              </h1>
              <div className="flex space-x-1">
                {tabs.map((tab) => (
                  <button
                    key={tab.id}
                    onClick={() => setActiveTab(tab.id)}
                    className={`px-4 py-2 rounded-lg transition-all duration-200 transform hover:scale-105 ${
                      activeTab === tab.id
                        ? 'bg-blue-600 text-white shadow-lg'
                        : isDark
                        ? 'text-gray-300 hover:bg-gray-700 hover:text-white'
                        : 'text-gray-600 hover:bg-gray-100 hover:text-gray-900'
                    }`}
                  >
                    <span className="mr-2">{tab.icon}</span>
                    {tab.label}
                    {tab.id === 'alerts' && alertCount > 0 && (
                      <span className="ml-2 px-2 py-0.5 text-xs font-bold bg-red-500 text-white rounded-full">
                        {alertCount}
                      </span>
                    )}
                  </button>
                ))}
              </div>
            </div>
            <div className="flex items-center space-x-4">
              <div 
                className={`text-sm transition-colors duration-300 ${
                  isDark ? 'text-gray-400' : 'text-gray-500'
                }`}
              >
                v0.2.0
              </div>
              <button
                onClick={handleThemeToggle}
                className={`p-2 rounded-lg transition-all duration-300 transform hover:scale-110 ${
                  isDark 
                    ? 'bg-gray-700 text-yellow-400 hover:bg-gray-600' 
                    : 'bg-gray-200 text-gray-700 hover:bg-gray-300'
                }`}
              >
                {isDark ? '☀️' : '🌙'}
              </button>
            </div>
          </div>
        </div>
      </nav>

      <main className="max-w-7xl mx-auto px-4 py-6">
        <div 
          className="animate-fade-in"
          key={activeTab}
        >
          {renderContent()}
        </div>
      </main>

      <Toaster 
        position="bottom-right"
        toastOptions={{
          duration: 4000,
          style: {
            background: isDark ? '#374151' : '#fff',
            color: isDark ? '#F3F4F6' : '#111',
          },
        }}
      />
    </div>
  );
}

const LoadingSkeleton: React.FC<{ isDark: boolean }> = ({ isDark }) => {
  return (
    <div className="space-y-6 animate-pulse">
      <div className="flex items-center justify-between">
        <div 
          className={`h-8 w-32 rounded ${
            isDark ? 'bg-gray-700' : 'bg-gray-200'
          }`}
        />
        <div 
          className={`h-10 w-24 rounded ${
            isDark ? 'bg-gray-700' : 'bg-gray-200'
          }`}
        />
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        {[1, 2, 3, 4].map((i) => (
          <div 
            key={i} 
            className={`rounded-xl p-6 ${
              isDark ? 'bg-gray-800' : 'bg-white'
            }`}
          >
            <div className="flex items-center justify-between mb-4">
              <div 
                className={`h-8 w-8 rounded-full ${
                  isDark ? 'bg-gray-700' : 'bg-gray-200'
                }`}
              />
              <div 
                className={`h-6 w-16 rounded ${
                  isDark ? 'bg-gray-700' : 'bg-gray-200'
                }`}
              />
            </div>
            <div 
              className={`h-10 w-20 rounded mb-2 ${
                isDark ? 'bg-gray-700' : 'bg-gray-200'
              }`}
            />
            <div 
              className={`h-4 w-32 rounded ${
                isDark ? 'bg-gray-700' : 'bg-gray-200'
              }`}
            />
          </div>
        ))}
      </div>

      <div 
        className={`rounded-xl p-6 ${
          isDark ? 'bg-gray-800' : 'bg-white'
        }`}
      >
        <div 
          className={`h-6 w-40 rounded mb-4 ${
            isDark ? 'bg-gray-700' : 'bg-gray-200'
          }`}
        />
        <div 
          className={`h-64 rounded ${
            isDark ? 'bg-gray-700' : 'bg-gray-200'
          }`}
        />
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div 
          className={`rounded-xl p-6 ${
            isDark ? 'bg-gray-800' : 'bg-white'
          }`}
        >
          <div 
            className={`h-6 w-32 rounded mb-4 ${
              isDark ? 'bg-gray-700' : 'bg-gray-200'
            }`}
          />
          <div className="space-y-3">
            {[1, 2, 3].map((i) => (
              <div 
                key={i} 
                className={`h-12 rounded ${
                  isDark ? 'bg-gray-700' : 'bg-gray-200'
                }`}
              />
            ))}
          </div>
        </div>
        <div 
          className={`rounded-xl p-6 ${
            isDark ? 'bg-gray-800' : 'bg-white'
          }`}
        >
          <div 
            className={`h-6 w-32 rounded mb-4 ${
              isDark ? 'bg-gray-700' : 'bg-gray-200'
            }`}
          />
          <div className="space-y-3">
            {[1, 2, 3].map((i) => (
              <div 
                key={i} 
                className={`h-12 rounded ${
                  isDark ? 'bg-gray-700' : 'bg-gray-200'
                }`}
              />
            ))}
          </div>
        </div>
      </div>
    </div>
  );
};

export default App;
