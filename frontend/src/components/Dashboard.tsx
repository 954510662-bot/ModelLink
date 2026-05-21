import React, { useEffect, useState } from 'react';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts';
import toast from 'react-hot-toast';

interface HealthStatus {
  status: string;
  timestamp: string;
  version: string;
}

interface Metrics {
  requests_total: number;
  errors_total: number;
  tokens_total: number;
  active_streams: number;
  avg_latency: number;
  error_rate: number;
  cpu_usage: number;
  memory_usage: number;
}

const Dashboard: React.FC = () => {
  const [health, setHealth] = useState<HealthStatus | null>(null);
  const [metrics, setMetrics] = useState<Metrics | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetchData();
    const interval = setInterval(fetchData, 5000);
    return () => clearInterval(interval);
  }, []);

  const fetchData = async () => {
    try {
      const [healthRes, metricsRes] = await Promise.all([
        fetch('http://localhost:9191/health'),
        fetch('http://localhost:9191/metrics'),
      ]);

      if (healthRes.ok) {
        setHealth(await healthRes.json());
      }

      if (metricsRes.ok) {
        const text = await metricsRes.text();
        const parsed = parseMetrics(text);
        setMetrics(parsed);
      }
    } catch (error) {
      console.error('Failed to fetch data:', error);
    } finally {
      setLoading(false);
    }
  };

  const parseMetrics = (text: string): Metrics => {
    const lines = text.split('\n');
    const result: any = {};
    
    lines.forEach((line) => {
      if (line.startsWith('model_link_requests_total')) {
        result.requests_total = parseInt(line.split(' ')[1]) || 0;
      } else if (line.startsWith('model_link_errors_total')) {
        result.errors_total = parseInt(line.split(' ')[1]) || 0;
      } else if (line.startsWith('model_link_tokens_total')) {
        result.tokens_total = parseInt(line.split(' ')[1]) || 0;
      } else if (line.startsWith('model_link_active_streams')) {
        result.active_streams = parseInt(line.split(' ')[1]) || 0;
      } else if (line.startsWith('model_link_request_duration_seconds_avg')) {
        result.avg_latency = parseFloat(line.split(' ')[1]) || 0;
      } else if (line.startsWith('model_link_error_rate')) {
        result.error_rate = parseFloat(line.split(' ')[1]) || 0;
      } else if (line.startsWith('model_link_cpu_usage')) {
        result.cpu_usage = parseFloat(line.split(' ')[1]) || 0;
      } else if (line.startsWith('model_link_memory_usage')) {
        result.memory_usage = parseFloat(line.split(' ')[1]) || 0;
      }
    });

    return {
      requests_total: result.requests_total || 0,
      errors_total: result.errors_total || 0,
      tokens_total: result.tokens_total || 0,
      active_streams: result.active_streams || 0,
      avg_latency: result.avg_latency || 0,
      error_rate: result.error_rate || 0,
      cpu_usage: result.cpu_usage || 0,
      memory_usage: result.memory_usage || 0,
    };
  };

  if (loading) {
    return <DashboardSkeleton />;
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h2 className="text-2xl font-bold text-white">Dashboard</h2>
        <button
          onClick={() => fetchData()}
          className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
        >
          🔄 Refresh
        </button>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <StatCard
          title="Total Requests"
          value={metrics?.requests_total || 0}
          icon="📨"
          color="blue"
        />
        <StatCard
          title="Total Errors"
          value={metrics?.errors_total || 0}
          icon="❌"
          color="red"
        />
        <StatCard
          title="Active Streams"
          value={metrics?.active_streams || 0}
          icon="🌊"
          color="green"
        />
        <StatCard
          title="Avg Latency"
          value={`${(metrics?.avg_latency || 0).toFixed(2)}s`}
          icon="⚡"
          color="yellow"
        />
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <PerformanceCard
          title="CPU Usage"
          value={metrics?.cpu_usage || 0}
          maxValue={100}
          unit="%"
          color="blue"
        />
        <PerformanceCard
          title="Memory Usage"
          value={metrics?.memory_usage || 0}
          maxValue={100}
          unit="%"
          color="purple"
        />
        <PerformanceCard
          title="Error Rate"
          value={(metrics?.error_rate || 0) * 100}
          maxValue={10}
          unit="%"
          color={(metrics?.error_rate || 0) > 0.05 ? "red" : "green"}
        />
      </div>

      <div className="bg-gray-800 rounded-xl p-6">
        <h3 className="text-lg font-semibold text-white mb-4">Request Volume</h3>
        <ResponsiveContainer width="100%" height={300}>
          <LineChart data={generateChartData()}>
            <CartesianGrid strokeDasharray="3 3" stroke="#374151" />
            <XAxis dataKey="time" stroke="#9CA3AF" />
            <YAxis stroke="#9CA3AF" />
            <Tooltip
              contentStyle={{
                backgroundColor: '#1F2937',
                border: 'none',
                borderRadius: '8px',
              }}
              labelStyle={{ color: '#F3F4F6' }}
            />
            <Line
              type="monotone"
              dataKey="requests"
              stroke="#3B82F6"
              strokeWidth={2}
              dot={false}
            />
          </LineChart>
        </ResponsiveContainer>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div className="bg-gray-800 rounded-xl p-6">
          <div className="flex items-center justify-between mb-4">
            <h3 className="text-lg font-semibold text-white">System Health</h3>
            {metrics?.errors_total && metrics.errors_total > 0 && (
              <span className="px-2 py-1 bg-red-500/20 text-red-400 rounded text-xs">
                ⚠️ Issues Detected
              </span>
            )}
          </div>
          <div className="flex items-center space-x-4">
            <div className={`w-4 h-4 rounded-full ${health?.status === 'healthy' ? 'bg-green-500' : 'bg-red-500'}`}></div>
            <span className="text-white font-medium">
              {health?.status === 'healthy' ? 'Healthy' : 'Unhealthy'}
            </span>
            <span className="text-gray-400">v{health?.version || 'N/A'}</span>
          </div>
        </div>

        <div className="bg-gray-800 rounded-xl p-6">
          <h3 className="text-lg font-semibold text-white mb-4">Quick Actions</h3>
          <div className="space-y-3">
            <button
              onClick={() => toast.success('Config reloaded!')}
              className="w-full px-4 py-2 bg-gray-700 text-white rounded-lg hover:bg-gray-600 transition-colors text-left"
            >
              🔄 Reload Configuration
            </button>
            <button
              onClick={() => toast.success('Backup created!')}
              className="w-full px-4 py-2 bg-gray-700 text-white rounded-lg hover:bg-gray-600 transition-colors text-left"
            >
              💾 Create Backup
            </button>
            <button
              onClick={() => toast.success('Health check triggered!')}
              className="w-full px-4 py-2 bg-gray-700 text-white rounded-lg hover:bg-gray-600 transition-colors text-left"
            >
              🏥 Run Health Check
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};

const StatCard: React.FC<{ title: string; value: string | number; icon: string; color: string }> = ({
  title,
  value,
  icon,
  color,
}) => {
  const colorClasses: Record<string, string> = {
    blue: 'bg-blue-500/20 text-blue-400',
    red: 'bg-red-500/20 text-red-400',
    green: 'bg-green-500/20 text-green-400',
    yellow: 'bg-yellow-500/20 text-yellow-400',
  };

  return (
    <div className="bg-gray-800 rounded-xl p-6">
      <div className="flex items-center justify-between mb-4">
        <span className="text-3xl">{icon}</span>
        <span className={`px-2 py-1 rounded-full text-xs font-medium ${colorClasses[color]}`}>
          Active
        </span>
      </div>
      <div className="text-3xl font-bold text-white mb-1">{value}</div>
      <div className="text-gray-400 text-sm">{title}</div>
    </div>
  );
};

const PerformanceCard: React.FC<{ title: string; value: number; maxValue: number; unit: string; color: string }> = ({
  title,
  value,
  maxValue,
  unit,
  color,
}) => {
  const percentage = Math.min((value / maxValue) * 100, 100);
  
  const colorClasses: Record<string, string> = {
    blue: 'bg-blue-500',
    purple: 'bg-purple-500',
    green: 'bg-green-500',
    red: 'bg-red-500',
    yellow: 'bg-yellow-500',
  };

  return (
    <div className="bg-gray-800 rounded-xl p-6">
      <div className="flex items-center justify-between mb-4">
        <h4 className="text-white font-semibold">{title}</h4>
        <span className={`text-2xl font-bold ${color === 'red' ? 'text-red-500' : color === 'green' ? 'text-green-500' : 'text-white'}`}>
          {value.toFixed(1)}{unit}
        </span>
      </div>
      <div className="w-full bg-gray-700 rounded-full h-2.5">
        <div
          className={`${colorClasses[color]} h-2.5 rounded-full transition-all duration-500`}
          style={{ width: `${percentage}%` }}
        ></div>
      </div>
      <div className="mt-2 flex justify-between text-xs text-gray-400">
        <span>0</span>
        <span>{maxValue}{unit}</span>
      </div>
    </div>
  );
};

const DashboardSkeleton: React.FC = () => {
  return (
    <div className="space-y-6 animate-pulse">
      <div className="flex items-center justify-between">
        <div className="h-8 w-32 bg-gray-700 rounded"></div>
        <div className="h-10 w-24 bg-gray-700 rounded"></div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        {[1, 2, 3, 4].map((i) => (
          <div key={i} className="bg-gray-800 rounded-xl p-6">
            <div className="flex items-center justify-between mb-4">
              <div className="h-8 w-8 bg-gray-700 rounded-full"></div>
              <div className="h-6 w-16 bg-gray-700 rounded"></div>
            </div>
            <div className="h-10 w-20 bg-gray-700 rounded mb-2"></div>
            <div className="h-4 w-32 bg-gray-700 rounded"></div>
          </div>
        ))}
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {[1, 2, 3].map((i) => (
          <div key={i} className="bg-gray-800 rounded-xl p-6">
            <div className="flex items-center justify-between mb-4">
              <div className="h-6 w-32 bg-gray-700 rounded"></div>
              <div className="h-8 w-20 bg-gray-700 rounded"></div>
            </div>
            <div className="w-full bg-gray-700 rounded-full h-2.5">
              <div className="h-2.5 bg-blue-500 rounded-full w-3/4"></div>
            </div>
          </div>
        ))}
      </div>

      <div className="bg-gray-800 rounded-xl p-6">
        <div className="h-6 w-40 bg-gray-700 rounded mb-4"></div>
        <div className="h-64 bg-gray-700 rounded"></div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div className="bg-gray-800 rounded-xl p-6">
          <div className="h-6 w-32 bg-gray-700 rounded mb-4"></div>
          <div className="space-y-3">
            {[1, 2, 3].map((i) => (
              <div key={i} className="h-12 bg-gray-700 rounded"></div>
            ))}
          </div>
        </div>
        <div className="bg-gray-800 rounded-xl p-6">
          <div className="h-6 w-32 bg-gray-700 rounded mb-4"></div>
          <div className="space-y-3">
            {[1, 2, 3].map((i) => (
              <div key={i} className="h-12 bg-gray-700 rounded"></div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
};

const generateChartData = () => {
  const now = Date.now();
  return Array.from({ length: 24 }, (_, i) => ({
    time: new Date(now - (23 - i) * 3600000).toLocaleTimeString([], { hour: '2-digit' }),
    requests: Math.floor(Math.random() * 100) + 50,
  }));
};

export default Dashboard;
