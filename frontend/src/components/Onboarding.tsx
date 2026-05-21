import React, { useState } from 'react';
import toast from 'react-hot-toast';

interface OnboardingProps {
  onComplete: () => void;
}

interface Step {
  id: string;
  title: string;
  description: string;
  icon: string;
  features: string[];
}

const steps: Step[] = [
  {
    id: 'welcome',
    title: 'Welcome to ModelLink',
    description: 'Your unified API gateway for AI model providers',
    icon: '🚀',
    features: [
      'Unified interface for multiple AI providers',
      'Automatic failover and load balancing',
      'Real-time monitoring and metrics',
      'Request/response transformation',
    ],
  },
  {
    id: 'providers',
    title: 'Connect Your Providers',
    description: 'Add your API keys and configure AI model providers',
    icon: '🔗',
    features: [
      'Support for OpenAI, Anthropic, Gemini, and more',
      'Easy API key configuration',
      'Automatic model capability detection',
      'Secure credential management',
    ],
  },
  {
    id: 'monitoring',
    title: 'Monitor Everything',
    description: 'Real-time insights into your API usage',
    icon: '📊',
    features: [
      'Request volume and latency tracking',
      'Error rate monitoring',
      'Token usage analytics',
      'Customizable dashboards',
    ],
  },
  {
    id: 'alerts',
    title: 'Stay Alerted',
    description: 'Get notified about issues before they become problems',
    icon: '🔔',
    features: [
      'Configurable alert rules',
      'Multiple notification channels',
      'Severity-based filtering',
      'Alert history and resolution tracking',
    ],
  },
  {
    id: 'complete',
    title: 'Ready to Go!',
    description: 'You\'re all set to start using ModelLink',
    icon: '✅',
    features: [
      'Access the dashboard',
      'Add more providers anytime',
      'Configure alerts',
      'Explore advanced features',
    ],
  },
];

const Onboarding: React.FC<OnboardingProps> = ({ onComplete }) => {
  const [currentStep, setCurrentStep] = useState(0);
  const [config, setConfig] = useState({
    provider: '',
    apiKey: '',
    providerName: '',
  });

  const handleNext = () => {
    if (currentStep < steps.length - 1) {
      setCurrentStep(currentStep + 1);
    } else {
      handleComplete();
    }
  };

  const handlePrevious = () => {
    if (currentStep > 0) {
      setCurrentStep(currentStep - 1);
    }
  };

  const handleComplete = () => {
    localStorage.setItem('modelLink-onboarding-completed', 'true');
    toast.success('Setup completed! Welcome to ModelLink');
    onComplete();
  };

  const handleSkip = () => {
    handleComplete();
  };

  const handleProviderConfig = () => {
    if (!config.provider || !config.apiKey) {
      toast.error('Please fill in all fields');
      return;
    }
    toast.success(`${config.providerName} configured successfully!`);
    handleNext();
  };

  const step = steps[currentStep];
  const progress = ((currentStep + 1) / steps.length) * 100;

  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-900 via-purple-900 to-gray-900 flex items-center justify-center p-8">
      <div className="max-w-4xl w-full">
        <div className="bg-gray-800 rounded-2xl shadow-2xl overflow-hidden">
          <div className="p-8">
            <div className="flex items-center justify-between mb-8">
              <h1 className="text-3xl font-bold text-white">ModelLink Setup</h1>
              <button
                onClick={handleSkip}
                className="text-gray-400 hover:text-white transition-colors"
              >
                Skip Setup
              </button>
            </div>

            <div className="mb-8">
              <div className="h-2 bg-gray-700 rounded-full overflow-hidden">
                <div
                  className="h-full bg-gradient-to-r from-blue-500 to-purple-500 transition-all duration-500"
                  style={{ width: `${progress}%` }}
                ></div>
              </div>
              <div className="flex justify-between mt-2">
                {steps.map((s, index) => (
                  <div
                    key={s.id}
                    className={`text-xs ${
                      index <= currentStep ? 'text-blue-400' : 'text-gray-500'
                    }`}
                  >
                    {index + 1}
                  </div>
                ))}
              </div>
            </div>

            <div className="text-center mb-8">
              <div className="text-6xl mb-4">{step.icon}</div>
              <h2 className="text-2xl font-bold text-white mb-2">{step.title}</h2>
              <p className="text-gray-400">{step.description}</p>
            </div>

            {currentStep === 1 && (
              <div className="mb-8 space-y-4">
                <div>
                  <label className="block text-sm font-medium text-gray-300 mb-2">
                    Provider Name
                  </label>
                  <input
                    type="text"
                    value={config.providerName}
                    onChange={(e) => setConfig({ ...config, providerName: e.target.value })}
                    placeholder="My OpenAI Provider"
                    className="w-full px-4 py-2 bg-gray-700 text-white rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-300 mb-2">
                    Provider Type
                  </label>
                  <select
                    value={config.provider}
                    onChange={(e) => setConfig({ ...config, provider: e.target.value })}
                    className="w-full px-4 py-2 bg-gray-700 text-white rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                  >
                    <option value="">Select a provider</option>
                    <option value="openai">OpenAI</option>
                    <option value="anthropic">Anthropic</option>
                    <option value="gemini">Google Gemini</option>
                    <option value="deepseek">DeepSeek</option>
                    <option value="mistral">Mistral AI</option>
                    <option value="groq">Groq</option>
                    <option value="ollama">Ollama</option>
                  </select>
                </div>
                {config.provider !== 'ollama' && (
                  <div>
                    <label className="block text-sm font-medium text-gray-300 mb-2">
                      API Key
                    </label>
                    <input
                      type="password"
                      value={config.apiKey}
                      onChange={(e) => setConfig({ ...config, apiKey: e.target.value })}
                      placeholder="sk-..."
                      className="w-full px-4 py-2 bg-gray-700 text-white rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                    />
                  </div>
                )}
              </div>
            )}

            <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-8">
              {step.features.map((feature, index) => (
                <div
                  key={index}
                  className="flex items-start space-x-3 p-3 bg-gray-700/50 rounded-lg"
                >
                  <span className="text-green-400 mt-1">✓</span>
                  <span className="text-gray-300">{feature}</span>
                </div>
              ))}
            </div>

            <div className="flex justify-between">
              <button
                onClick={handlePrevious}
                disabled={currentStep === 0}
                className={`px-6 py-2 rounded-lg transition-colors ${
                  currentStep === 0
                    ? 'bg-gray-700 text-gray-500 cursor-not-allowed'
                    : 'bg-gray-700 text-white hover:bg-gray-600'
                }`}
              >
                Previous
              </button>
              
              {currentStep === 1 ? (
                <button
                  onClick={handleProviderConfig}
                  className="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
                >
                  Add Provider
                </button>
              ) : (
                <button
                  onClick={handleNext}
                  className="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
                >
                  {currentStep === steps.length - 1 ? 'Get Started' : 'Next'}
                </button>
              )}
            </div>
          </div>
        </div>

        <div className="mt-8 text-center">
          <p className="text-gray-500 text-sm">
            Need help? Check out our{' '}
            <a href="#" className="text-blue-400 hover:underline">
              documentation
            </a>{' '}
            or{' '}
            <a href="#" className="text-blue-400 hover:underline">
              contact support
            </a>
          </p>
        </div>
      </div>
    </div>
  );
};

export default Onboarding;
