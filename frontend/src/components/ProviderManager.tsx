import React, { useState, useEffect } from 'react';
import toast from 'react-hot-toast';

interface Provider {
  name: string;
  base_url: string;
  status: 'healthy' | 'unhealthy' | 'unknown';
  requests: number;
  errors: number;
}

const ProviderManager: React.FC = () => {
  const [providers, setProviders] = useState<Provider[]>([
    { name: 'deepseek', base_url: 'https://api.deepseek.com/v1', status: 'healthy', requests: 1542, errors: 12 },
    { name: 'openai', base_url: 'https://api.openai.com/v1', status: 'healthy', requests: 892, errors: 5 },
    { name: 'anthropic', base_url: 'https://api.anthropic.com/v1', status: 'unhealthy', requests: 234, errors: 45 },
  ]);
  const [showAddModal, setShowAddModal] = useState(false);

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h2 className="text-2xl font-bold text-white">Provider Management</h2>
        <button
          onClick={() => setShowAddModal(true)}
          className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
        >
          ➕ Add Provider
        </button>
      </div>

      <div className="grid grid-cols-1 gap-4">
        {providers.map((provider) => (
          <div key={provider.name} className="bg-gray-800 rounded-xl p-6">
            <div className="flex items-center justify-between">
              <div className="flex items-center space-x-4">
                <div className={`w-3 h-3 rounded-full ${
                  provider.status === 'healthy' ? 'bg-green-500' :
                  provider.status === 'unhealthy' ? 'bg-red-500' : 'bg-gray-500'
                }`}></div>
                <div>
                  <h3 className="text-lg font-semibold text-white">{provider.name}</h3>
                  <p className="text-sm text-gray-400">{provider.base_url}</p>
                </div>
              </div>
              <div className="flex items-center space-x-6">
                <div className="text-right">
                  <div className="text-2xl font-bold text-white">{provider.requests}</div>
                  <div className="text-sm text-gray-400">Requests</div>
                </div>
                <div className="text-right">
                  <div className="text-2xl font-bold text-red-400">{provider.errors}</div>
                  <div className="text-sm text-gray-400">Errors</div>
                </div>
                <div className="flex space-x-2">
                  <button
                    onClick={() => toast.success(`Testing ${provider.name}...`)}
                    className="px-3 py-2 bg-gray-700 text-white rounded-lg hover:bg-gray-600 transition-colors"
                  >
                    🧪 Test
                  </button>
                  <button
                    onClick={() => toast.success(`Switched to ${provider.name}`)}
                    className="px-3 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
                  >
                    🔄 Switch
                  </button>
                  <button
                    onClick={() => toast.error(`Removed ${provider.name}`)}
                    className="px-3 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 transition-colors"
                  >
                    🗑️
                  </button>
                </div>
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

export default ProviderManager;
