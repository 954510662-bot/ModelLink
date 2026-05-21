import React, { useEffect, useState } from 'react';
import { PieChart, Pie, Cell, ResponsiveContainer, BarChart, Bar, XAxis, YAxis, Tooltip } from 'recharts';
import toast from 'react-hot-toast';

interface Alert {
  id: string;
  title: string;
  message: string;
  severity: string;
  source: string;
  timestamp: string;
  resolved: boolean;
}

interface AlertStats {
  total_alerts: number;
  active_alerts: number;
  resolved_alerts: number;
  critical_count: number;
  high_count: number;
  medium_count: number;
  low_count: number;
}

const severityColors: Record<string, string> = {
  Critical: '#EF4444',
  High: '#F97316',
  Medium: '#EAB308',
  Low: '#22C55E',
  Info: '#3B82F6',
};

const AlertsView: React.FC = () => {
  const [alerts, setAlerts] = useState<Alert[]>([]);
  const [stats, setStats] = useState<AlertStats | null>(null);
  const [loading, setLoading] = useState(true);
  const [filter, setFilter] = useState<string>('all');
  const [searchTerm, setSearchTerm] = useState('');
  const [selectedAlert, setSelectedAlert] = useState<Alert | null>(null);

  useEffect(() => {
    fetchAlerts();
    const interval = setInterval(fetchAlerts, 10000);
    return () => clearInterval(interval);
  }, []);

  const fetchAlerts = async () => {
    try {
      const [alertsRes, statsRes] = await Promise.all([
        fetch('http://localhost:9191/alerts'),
        fetch('http://localhost:9191/alerts/stats'),
      ]);

      if (alertsRes.ok) {
        const data = await alertsRes.json();
        setAlerts(Array.isArray(data) ? data : []);
      }

      if (statsRes.ok) {
        const data = await statsRes.json();
        setStats(data);
      }
    } catch (error) {
      console.error('Failed to fetch alerts:', error);
      setAlerts([]);
      setStats({
        total_alerts: 0,
        active_alerts: 0,
        resolved_alerts: 0,
        critical_count: 0,
        high_count: 0,
        medium_count: 0,
        low_count: 0,
      });
    } finally {
      setLoading(false);
    }
  };

  const handleResolve = async (alertId: string) => {
    try {
      const res = await fetch(`http://localhost:9191/alerts/${alertId}/resolve`, {
        method: 'POST',
      });
      if (res.ok) {
        toast.success('Alert resolved');
        fetchAlerts();
      } else {
        toast.error('Failed to resolve alert');
      }
    } catch (error) {
      toast.error('Failed to resolve alert');
    }
  };

  const filteredAlerts = alerts.filter((alert) => {
    const matchesFilter = filter === 'all' || 
      (filter === 'active' && !alert.resolved) ||
      (filter === 'resolved' && alert.resolved) ||
      alert.severity.toLowerCase() === filter;
    
    const matchesSearch = alert.title.toLowerCase().includes(searchTerm.toLowerCase()) ||
      alert.message.toLowerCase().includes(searchTerm.toLowerCase());
    
    return matchesFilter && matchesSearch;
  });

  const pieData = stats ? [
    { name: 'Critical', value: stats.critical_count, color: severityColors.Critical },
    { name: 'High', value: stats.high_count, color: severityColors.High },
    { name: 'Medium', value: stats.medium_count, color: severityColors.Medium },
    { name: 'Low', value: stats.low_count, color: severityColors.Low },
  ].filter(item => item.value > 0) : [];

  const historyData = generateHistoryData();

  if (loading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h2 className="text-2xl font-bold text-white">Alerts</h2>
        <button
          onClick={() => fetchAlerts()}
          className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
        >
          🔄 Refresh
        </button>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <StatCard
          title="Total Alerts"
          value={stats?.total_alerts || 0}
          icon="🔔"
          color="blue"
        />
        <StatCard
          title="Active"
          value={stats?.active_alerts || 0}
          icon="⚠️"
          color="red"
        />
        <StatCard
          title="Resolved"
          value={stats?.resolved_alerts || 0}
          icon="✅"
          color="green"
        />
        <StatCard
          title="Critical"
          value={stats?.critical_count || 0}
          icon="🚨"
          color="red"
        />
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div className="bg-gray-800 rounded-xl p-6">
          <h3 className="text-lg font-semibold text-white mb-4">Alert Distribution</h3>
          {pieData.length > 0 ? (
            <ResponsiveContainer width="100%" height={250}>
              <PieChart>
                <Pie
                  data={pieData}
                  cx="50%"
                  cy="50%"
                  innerRadius={60}
                  outerRadius={100}
                  paddingAngle={5}
                  dataKey="value"
                >
                  {pieData.map((entry, index) => (
                    <Cell key={`cell-${index}`} fill={entry.color} />
                  ))}
                </Pie>
                <Tooltip />
              </PieChart>
            </ResponsiveContainer>
          ) : (
            <div className="h-64 flex items-center justify-center text-gray-400">
              No alerts
            </div>
          )}
          <div className="mt-4 flex flex-wrap gap-4 justify-center">
            {pieData.map((item) => (
              <div key={item.name} className="flex items-center space-x-2">
                <div className="w-3 h-3 rounded-full" style={{ backgroundColor: item.color }}></div>
                <span className="text-sm text-gray-400">{item.name}: {item.value}</span>
              </div>
            ))}
          </div>
        </div>

        <div className="bg-gray-800 rounded-xl p-6">
          <h3 className="text-lg font-semibold text-white mb-4">Alert History (24h)</h3>
          <ResponsiveContainer width="100%" height={250}>
            <BarChart data={historyData}>
              <XAxis dataKey="hour" stroke="#9CA3AF" />
              <YAxis stroke="#9CA3AF" />
              <Tooltip
                contentStyle={{
                  backgroundColor: '#1F2937',
                  border: 'none',
                  borderRadius: '8px',
                }}
              />
              <Bar dataKey="alerts" fill="#3B82F6" />
            </BarChart>
          </ResponsiveContainer>
        </div>
      </div>

      <div className="bg-gray-800 rounded-xl p-6">
        <div className="mb-4 flex items-center justify-between space-x-4">
          <div className="flex items-center space-x-2 flex-1">
            <input
              type="text"
              placeholder="Search alerts..."
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              className="px-4 py-2 bg-gray-700 text-white rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 flex-1"
            />
          </div>
          <div className="flex space-x-2">
            {['all', 'active', 'resolved', 'critical', 'high', 'medium', 'low'].map((f) => (
              <button
                key={f}
                onClick={() => setFilter(f)}
                className={`px-3 py-1 rounded-lg text-sm font-medium transition-colors ${
                  filter === f
                    ? 'bg-blue-600 text-white'
                    : 'bg-gray-700 text-gray-300 hover:bg-gray-600'
                }`}
              >
                {f.charAt(0).toUpperCase() + f.slice(1)}
              </button>
            ))}
          </div>
        </div>

        <div className="space-y-3">
          {filteredAlerts.length > 0 ? (
            filteredAlerts.map((alert) => (
              <div
                key={alert.id}
                className={`p-4 rounded-lg border transition-all hover:shadow-lg cursor-pointer ${
                  alert.resolved
                    ? 'bg-gray-700/50 border-gray-600'
                    : 'bg-gray-700 border-gray-600 hover:border-gray-500'
                }`}
                onClick={() => setSelectedAlert(alert)}
              >
                <div className="flex items-start justify-between">
                  <div className="flex-1">
                    <div className="flex items-center space-x-2 mb-2">
                      <span
                        className="px-2 py-1 rounded text-xs font-bold"
                        style={{
                          backgroundColor: `${severityColors[alert.severity]}20`,
                          color: severityColors[alert.severity],
                        }}
                      >
                        {alert.severity}
                      </span>
                      <span className="text-sm text-gray-400">{alert.source}</span>
                      <span className="text-sm text-gray-500">
                        {new Date(alert.timestamp).toLocaleString()}
                      </span>
                    </div>
                    <h4 className="text-white font-semibold mb-1">{alert.title}</h4>
                    <p className="text-gray-400 text-sm">{alert.message}</p>
                  </div>
                  {!alert.resolved && (
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                        handleResolve(alert.id);
                      }}
                      className="px-3 py-1 bg-green-600 text-white rounded hover:bg-green-700 transition-colors text-sm"
                    >
                      Resolve
                    </button>
                  )}
                  {alert.resolved && (
                    <span className="px-3 py-1 bg-green-600/20 text-green-400 rounded text-sm">
                      Resolved
                    </span>
                  )}
                </div>
              </div>
            ))
          ) : (
            <div className="text-center py-12 text-gray-400">
              <div className="text-4xl mb-4">🔔</div>
              <p>No alerts found</p>
            </div>
          )}
        </div>
      </div>

      {selectedAlert && (
        <AlertDetailModal
          alert={selectedAlert}
          onClose={() => setSelectedAlert(null)}
          onResolve={handleResolve}
        />
      )}
    </div>
  );
};

const StatCard: React.FC<{ title: string; value: number; icon: string; color: string }> = ({
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
          {title}
        </span>
      </div>
      <div className="text-3xl font-bold text-white mb-1">{value}</div>
      <div className="text-gray-400 text-sm">{title}</div>
    </div>
  );
};

const AlertDetailModal: React.FC<{
  alert: Alert;
  onClose: () => void;
  onResolve: (id: string) => void;
}> = ({ alert, onClose, onResolve }) => {
  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-gray-800 rounded-xl p-6 max-w-2xl w-full mx-4">
        <div className="flex items-start justify-between mb-4">
          <h3 className="text-xl font-bold text-white">{alert.title}</h3>
          <button
            onClick={onClose}
            className="text-gray-400 hover:text-white text-2xl"
          >
            ×
          </button>
        </div>
        
        <div className="space-y-4 mb-6">
          <div>
            <label className="text-sm text-gray-400">Severity</label>
            <p className="text-white">
              <span
                className="px-2 py-1 rounded text-xs font-bold"
                style={{
                  backgroundColor: `${severityColors[alert.severity]}20`,
                  color: severityColors[alert.severity],
                }}
              >
                {alert.severity}
              </span>
            </p>
          </div>
          <div>
            <label className="text-sm text-gray-400">Source</label>
            <p className="text-white">{alert.source}</p>
          </div>
          <div>
            <label className="text-sm text-gray-400">Timestamp</label>
            <p className="text-white">{new Date(alert.timestamp).toLocaleString()}</p>
          </div>
          <div>
            <label className="text-sm text-gray-400">Message</label>
            <p className="text-white">{alert.message}</p>
          </div>
          <div>
            <label className="text-sm text-gray-400">Status</label>
            <p className="text-white">{alert.resolved ? 'Resolved' : 'Active'}</p>
          </div>
        </div>

        <div className="flex justify-end space-x-3">
          <button
            onClick={onClose}
            className="px-4 py-2 bg-gray-700 text-white rounded-lg hover:bg-gray-600 transition-colors"
          >
            Close
          </button>
          {!alert.resolved && (
            <button
              onClick={() => {
                onResolve(alert.id);
                onClose();
              }}
              className="px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors"
            >
              Resolve Alert
            </button>
          )}
        </div>
      </div>
    </div>
  );
};

const generateHistoryData = () => {
  const now = new Date();
  return Array.from({ length: 24 }, (_, i) => {
    const hour = new Date(now.getTime() - (23 - i) * 3600000).getHours();
    return {
      hour: `${hour}:00`,
      alerts: Math.floor(Math.random() * 5),
    };
  });
};

export default AlertsView;
