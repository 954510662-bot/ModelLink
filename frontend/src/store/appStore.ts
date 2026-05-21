import { create } from 'zustand';
import { persist, createJSONStorage } from 'zustand/middleware';

interface Alert {
  id: string;
  title: string;
  message: string;
  severity: 'Critical' | 'High' | 'Medium' | 'Low' | 'Info';
  source: string;
  timestamp: string;
  resolved: boolean;
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

interface Provider {
  id: string;
  name: string;
  enabled: boolean;
  models: string[];
  config: Record<string, any>;
}

interface AppState {
  theme: 'light' | 'dark';
  sidebarOpen: boolean;
  activeTab: string;
  
  alerts: Alert[];
  alertCount: number;
  
  metrics: Metrics | null;
  
  providers: Provider[];
  
  notifications: Notification[];
  
  settings: {
    autoRefresh: boolean;
    refreshInterval: number;
    notifications: {
      enabled: boolean;
      sound: boolean;
      desktop: boolean;
    };
  };
}

interface AppActions {
  setTheme: (theme: 'light' | 'dark') => void;
  toggleTheme: () => void;
  setSidebarOpen: (open: boolean) => void;
  setActiveTab: (tab: string) => void;
  
  setAlerts: (alerts: Alert[]) => void;
  addAlert: (alert: Alert) => void;
  removeAlert: (id: string) => void;
  clearAlerts: () => void;
  setAlertCount: (count: number) => void;
  
  setMetrics: (metrics: Metrics) => void;
  
  setProviders: (providers: Provider[]) => void;
  addProvider: (provider: Provider) => void;
  updateProvider: (id: string, updates: Partial<Provider>) => void;
  removeProvider: (id: string) => void;
  
  addNotification: (notification: Omit<Notification, 'id' | 'timestamp'>) => void;
  removeNotification: (id: string) => void;
  clearNotifications: () => void;
  
  updateSettings: (settings: Partial<AppState['settings']>) => void;
  
  resetStore: () => void;
}

interface Notification {
  id: string;
  title: string;
  message: string;
  type: 'success' | 'error' | 'warning' | 'info';
  timestamp: number;
  read: boolean;
}

const initialState: AppState = {
  theme: 'dark',
  sidebarOpen: true,
  activeTab: 'dashboard',
  
  alerts: [],
  alertCount: 0,
  
  metrics: null,
  
  providers: [],
  
  notifications: [],
  
  settings: {
    autoRefresh: true,
    refreshInterval: 5000,
    notifications: {
      enabled: true,
      sound: true,
      desktop: false,
    },
  },
};

export const useAppStore = create<AppState & AppActions>()(
  persist(
    (set, get) => ({
      ...initialState,
      
      setTheme: (theme) => {
        set({ theme });
        localStorage.setItem('modelLink-theme', theme);
      },
      
      toggleTheme: () => {
        const newTheme = get().theme === 'dark' ? 'light' : 'dark';
        set({ theme: newTheme });
        localStorage.setItem('modelLink-theme', newTheme);
      },
      
      setSidebarOpen: (open) => set({ sidebarOpen: open }),
      
      setActiveTab: (tab) => set({ activeTab: tab }),
      
      setAlerts: (alerts) => set({ alerts, alertCount: alerts.filter(a => !a.resolved).length }),
      
      addAlert: (alert) => {
        const alerts = [alert, ...get().alerts].slice(0, 100);
        set({ alerts, alertCount: alerts.filter(a => !a.resolved).length });
      },
      
      removeAlert: (id) => {
        const alerts = get().alerts.filter(a => a.id !== id);
        set({ alerts, alertCount: alerts.filter(a => !a.resolved).length });
      },
      
      clearAlerts: () => set({ alerts: [], alertCount: 0 }),
      
      setAlertCount: (count) => set({ alertCount: count }),
      
      setMetrics: (metrics) => set({ metrics }),
      
      setProviders: (providers) => set({ providers }),
      
      addProvider: (provider) => {
        const providers = [...get().providers, provider];
        set({ providers });
      },
      
      updateProvider: (id, updates) => {
        const providers = get().providers.map(p => 
          p.id === id ? { ...p, ...updates } : p
        );
        set({ providers });
      },
      
      removeProvider: (id) => {
        const providers = get().providers.filter(p => p.id !== id);
        set({ providers });
      },
      
      addNotification: (notification) => {
        const newNotification: Notification = {
          ...notification,
          id: `notif-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`,
          timestamp: Date.now(),
          read: false,
        };
        const notifications = [newNotification, ...get().notifications].slice(0, 50);
        set({ notifications });
      },
      
      removeNotification: (id) => {
        const notifications = get().notifications.filter(n => n.id !== id);
        set({ notifications });
      },
      
      clearNotifications: () => set({ notifications: [] }),
      
      updateSettings: (settings) => {
        const currentSettings = get().settings;
        set({ settings: { ...currentSettings, ...settings } });
      },
      
      resetStore: () => set(initialState),
    }),
    {
      name: 'modelLink-storage',
      storage: createJSONStorage(() => localStorage),
      partialize: (state) => ({
        theme: state.theme,
        sidebarOpen: state.sidebarOpen,
        providers: state.providers,
        settings: state.settings,
      }),
    }
  )
);

export const useTheme = () => {
  const theme = useAppStore((state) => state.theme);
  const toggleTheme = useAppStore((state) => state.toggleTheme);
  return { theme, toggleTheme, isDark: theme === 'dark' };
};

export const useAlerts = () => {
  const alerts = useAppStore((state) => state.alerts);
  const alertCount = useAppStore((state) => state.alertCount);
  const setAlerts = useAppStore((state) => state.setAlerts);
  const addAlert = useAppStore((state) => state.addAlert);
  const removeAlert = useAppStore((state) => state.removeAlert);
  const clearAlerts = useAppStore((state) => state.clearAlerts);
  return { alerts, alertCount, setAlerts, addAlert, removeAlert, clearAlerts };
};

export const useMetrics = () => {
  const metrics = useAppStore((state) => state.metrics);
  const setMetrics = useAppStore((state) => state.setMetrics);
  return { metrics, setMetrics };
};

export const useProviders = () => {
  const providers = useAppStore((state) => state.providers);
  const setProviders = useAppStore((state) => state.setProviders);
  const addProvider = useAppStore((state) => state.addProvider);
  const updateProvider = useAppStore((state) => state.updateProvider);
  const removeProvider = useAppStore((state) => state.removeProvider);
  return { providers, setProviders, addProvider, updateProvider, removeProvider };
};

export const useNotifications = () => {
  const notifications = useAppStore((state) => state.notifications);
  const addNotification = useAppStore((state) => state.addNotification);
  const removeNotification = useAppStore((state) => state.removeNotification);
  const clearNotifications = useAppStore((state) => state.clearNotifications);
  return { notifications, addNotification, removeNotification, clearNotifications };
};

export const useSettings = () => {
  const settings = useAppStore((state) => state.settings);
  const updateSettings = useAppStore((state) => state.updateSettings);
  return { settings, updateSettings };
};
