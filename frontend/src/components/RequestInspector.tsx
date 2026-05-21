import React, { useState, useEffect } from 'react';

interface Request {
  id: string;
  timestamp: string;
  method: string;
  endpoint: string;
  status: number;
  duration: number;
  provider: string;
}

const RequestInspector: React.FC = () => {
  const [requests, setRequests] = useState<Request[]>([]);
  const [selectedRequest, setSelectedRequest] = useState<Request | null>(null);

  useEffect(() => {
    const mockRequests: Request[] = Array.from({ length: 20 }, (_, i) => ({
      id: `req-${i.toString().padStart(6, '0')}`,
      timestamp: new Date(Date.now() - i * 60000).toISOString(),
      method: i % 3 === 0 ? 'POST' : 'GET',
      endpoint: i % 3 === 0 ? '/v1/chat/completions' : '/health',
      status: i % 10 === 0 ? 500 : 200,
      duration: Math.floor(Math.random() * 500) + 50,
      provider: ['deepseek', 'openai', 'anthropic'][i % 3],
    }));
    setRequests(mockRequests);
  }, []);

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h2 className="text-2xl font-bold text-white">Request Inspector</h2>
        <div className="flex space-x-2">
          <button className="px-4 py-2 bg-gray-700 text-white rounded-lg hover:bg-gray-600">
            🔍 Filter
          </button>
          <button className="px-4 py-2 bg-gray-700 text-white rounded-lg hover:bg-gray-600">
            📥 Export
          </button>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div className="bg-gray-800 rounded-xl overflow-hidden">
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead className="bg-gray-700">
                <tr>
                  <th className="px-4 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">ID</th>
                  <th className="px-4 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">Time</th>
                  <th className="px-4 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">Method</th>
                  <th className="px-4 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">Endpoint</th>
                  <th className="px-4 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">Status</th>
                  <th className="px-4 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">Duration</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-gray-700">
                {requests.map((req) => (
                  <tr
                    key={req.id}
                    onClick={() => setSelectedRequest(req)}
                    className={`cursor-pointer hover:bg-gray-700 ${selectedRequest?.id === req.id ? 'bg-gray-700' : ''}`}
                  >
                    <td className="px-4 py-3 text-sm text-gray-300">{req.id}</td>
                    <td className="px-4 py-3 text-sm text-gray-300">
                      {new Date(req.timestamp).toLocaleTimeString()}
                    </td>
                    <td className="px-4 py-3 text-sm">
                      <span className={`px-2 py-1 rounded text-xs font-medium ${
                        req.method === 'POST' ? 'bg-blue-500/20 text-blue-400' : 'bg-green-500/20 text-green-400'
                      }`}>
                        {req.method}
                      </span>
                    </td>
                    <td className="px-4 py-3 text-sm text-gray-300">{req.endpoint}</td>
                    <td className="px-4 py-3 text-sm">
                      <span className={`px-2 py-1 rounded text-xs font-medium ${
                        req.status === 200 ? 'bg-green-500/20 text-green-400' : 'bg-red-500/20 text-red-400'
                      }`}>
                        {req.status}
                      </span>
                    </td>
                    <td className="px-4 py-3 text-sm text-gray-300">{req.duration}ms</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>

        <div className="bg-gray-800 rounded-xl p-6">
          <h3 className="text-lg font-semibold text-white mb-4">Request Details</h3>
          {selectedRequest ? (
            <div className="space-y-4">
              <div>
                <div className="text-sm text-gray-400 mb-1">Request ID</div>
                <div className="text-white font-mono">{selectedRequest.id}</div>
              </div>
              <div>
                <div className="text-sm text-gray-400 mb-1">Timestamp</div>
                <div className="text-white">{new Date(selectedRequest.timestamp).toLocaleString()}</div>
              </div>
              <div>
                <div className="text-sm text-gray-400 mb-1">Provider</div>
                <div className="text-white">{selectedRequest.provider}</div>
              </div>
              <div>
                <div className="text-sm text-gray-400 mb-1">Request Body</div>
                <pre className="bg-gray-900 p-4 rounded-lg text-sm text-gray-300 overflow-x-auto">
{JSON.stringify({
  model: "gpt-4",
  messages: [{ role: "user", content: "Hello!" }],
  temperature: 0.7
}, null, 2)}
                </pre>
              </div>
            </div>
          ) : (
            <div className="text-gray-400 text-center py-8">
              Select a request to view details
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

export default RequestInspector;
