import React from 'react';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer, AreaChart, Area } from 'recharts';

const MetricsView: React.FC = () => {
  const latencyData = generateLatencyData();
  const throughputData = generateThroughputData();

  return (
    <div className="space-y-6">
      <h2 className="text-2xl font-bold text-white">Metrics</h2>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div className="bg-gray-800 rounded-xl p-6">
          <h3 className="text-lg font-semibold text-white mb-4">Request Latency (ms)</h3>
          <ResponsiveContainer width="100%" height={250}>
            <AreaChart data={latencyData}>
              <CartesianGrid strokeDasharray="3 3" stroke="#374151" />
              <XAxis dataKey="time" stroke="#9CA3AF" />
              <YAxis stroke="#9CA3AF" />
              <Tooltip
                contentStyle={{ backgroundColor: '#1F2937', border: 'none', borderRadius: '8px' }}
                labelStyle={{ color: '#F3F4F6' }}
              />
              <Area type="monotone" dataKey="latency" stroke="#3B82F6" fill="#3B82F6" fillOpacity={0.3} />
            </AreaChart>
          </ResponsiveContainer>
        </div>

        <div className="bg-gray-800 rounded-xl p-6">
          <h3 className="text-lg font-semibold text-white mb-4">Request Throughput</h3>
          <ResponsiveContainer width="100%" height={250}>
            <AreaChart data={throughputData}>
              <CartesianGrid strokeDasharray="3 3" stroke="#374151" />
              <XAxis dataKey="time" stroke="#9CA3AF" />
              <YAxis stroke="#9CA3AF" />
              <Tooltip
                contentStyle={{ backgroundColor: '#1F2937', border: 'none', borderRadius: '8px' }}
                labelStyle={{ color: '#F3F4F6' }}
              />
              <Area type="monotone" dataKey="throughput" stroke="#10B981" fill="#10B981" fillOpacity={0.3} />
            </AreaChart>
          </ResponsiveContainer>
        </div>
      </div>

      <div className="bg-gray-800 rounded-xl p-6">
        <h3 className="text-lg font-semibold text-white mb-4">Error Rate</h3>
        <div className="grid grid-cols-4 gap-4">
          <MetricCard title="4xx Errors" value={23} color="yellow" />
          <MetricCard title="5xx Errors" value={5} color="red" />
          <MetricCard title="Timeouts" value={12} color="orange" />
          <MetricCard title="Success Rate" value="99.2%" color="green" />
        </div>
      </div>

      <div className="bg-gray-800 rounded-xl p-6">
        <h3 className="text-lg font-semibold text-white mb-4">Token Usage</h3>
        <div className="grid grid-cols-3 gap-4">
          <MetricCard title="Prompt Tokens" value="125,432" color="blue" />
          <MetricCard title="Completion Tokens" value="89,231" color="purple" />
          <MetricCard title="Total Tokens" value="214,663" color="cyan" />
        </div>
      </div>
    </div>
  );
};

const MetricCard: React.FC<{ title: string; value: string | number; color: string }> = ({ title, value, color }) => {
  const colorClasses: Record<string, string> = {
    blue: 'border-blue-500',
    red: 'border-red-500',
    green: 'border-green-500',
    yellow: 'border-yellow-500',
    orange: 'border-orange-500',
    purple: 'border-purple-500',
    cyan: 'border-cyan-500',
  };

  return (
    <div className={`bg-gray-900 rounded-lg p-4 border-l-4 ${colorClasses[color]}`}>
      <div className="text-2xl font-bold text-white">{value}</div>
      <div className="text-sm text-gray-400">{title}</div>
    </div>
  );
};

const generateLatencyData = () => {
  return Array.from({ length: 60 }, (_, i) => ({
    time: `${i}`,
    latency: Math.floor(Math.random() * 200) + 50,
  }));
};

const generateThroughputData = () => {
  return Array.from({ length: 60 }, (_, i) => ({
    time: `${i}`,
    throughput: Math.floor(Math.random() * 50) + 10,
  }));
};

export default MetricsView;
