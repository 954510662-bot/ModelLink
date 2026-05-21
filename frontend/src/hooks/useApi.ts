import { useState, useEffect, useCallback, useRef } from 'react';

interface CacheItem<T> {
  data: T;
  timestamp: number;
}

interface UseApiOptions<T> {
  url: string;
  method?: 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH';
  body?: any;
  headers?: Record<string, string>;
  enabled?: boolean;
  cacheTime?: number;
  retryCount?: number;
  retryDelay?: number;
  onSuccess?: (data: T) => void;
  onError?: (error: Error) => void;
}

interface UseApiReturn<T> {
  data: T | null;
  loading: boolean;
  error: Error | null;
  refetch: () => Promise<void>;
  isFetching: boolean;
}

const cache = new Map<string, CacheItem<any>>();
const MAX_CACHE_SIZE = 50;
const DEFAULT_CACHE_TIME = 5 * 60 * 1000;

function clearOldCache() {
  if (cache.size > MAX_CACHE_SIZE) {
    const entries = Array.from(cache.entries());
    entries.sort((a, b) => a[1].timestamp - b[1].timestamp);
    const toDelete = entries.slice(0, Math.floor(MAX_CACHE_SIZE * 0.3));
    toDelete.forEach(([key]) => cache.delete(key));
  }
}

function getCacheKey(url: string, method: string, body?: any): string {
  return `${method}:${url}:${JSON.stringify(body || {})}`;
}

function getFromCache<T>(key: string, cacheTime: number): T | null {
  const item = cache.get(key);
  if (item && Date.now() - item.timestamp < cacheTime) {
    return item.data as T;
  }
  return null;
}

function setCache<T>(key: string, data: T) {
  cache.set(key, { data, timestamp: Date.now() });
  clearOldCache();
}

async function fetchWithRetry<T>(
  url: string,
  options: RequestInit,
  retryCount: number,
  retryDelay: number
): Promise<T> {
  let lastError: Error;
  
  for (let i = 0; i <= retryCount; i++) {
    try {
      const response = await fetch(url, options);
      
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      
      if (response.status === 204) {
        return null as T;
      }
      
      return await response.json();
    } catch (error) {
      lastError = error as Error;
      
      if (i < retryCount) {
        await new Promise(resolve => setTimeout(resolve, retryDelay * Math.pow(2, i)));
      }
    }
  }
  
  throw lastError!;
}

export function useApi<T = any>(options: UseApiOptions<T>): UseApiReturn<T> {
  const {
    url,
    method = 'GET',
    body,
    headers = {},
    enabled = true,
    cacheTime = DEFAULT_CACHE_TIME,
    retryCount = 3,
    retryDelay = 1000,
    onSuccess,
    onError,
  } = options;

  const [data, setData] = useState<T | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<Error | null>(null);
  const [isFetching, setIsFetching] = useState(false);
  
  const mountedRef = useRef(true);
  const fetchCountRef = useRef(0);

  const fetchData = useCallback(async () => {
    if (!enabled || !url) return;
    
    const cacheKey = getCacheKey(url, method, body);
    const cachedData = getFromCache<T>(cacheKey, cacheTime);
    
    if (cachedData !== null) {
      setData(cachedData);
      onSuccess?.(cachedData);
      return;
    }

    setIsFetching(true);
    fetchCountRef.current += 1;
    const currentFetch = fetchCountRef.current;

    try {
      setLoading(true);
      setError(null);

      const requestOptions: RequestInit = {
        method,
        headers: {
          'Content-Type': 'application/json',
          ...headers,
        },
      };

      if (body && method !== 'GET') {
        requestOptions.body = JSON.stringify(body);
      }

      const result = await fetchWithRetry<T>(url, requestOptions, retryCount, retryDelay);

      if (mountedRef.current && currentFetch === fetchCountRef.current) {
        setData(result);
        setCache(cacheKey, result);
        onSuccess?.(result);
      }
    } catch (err) {
      if (mountedRef.current && currentFetch === fetchCountRef.current) {
        const error = err instanceof Error ? err : new Error('An error occurred');
        setError(error);
        onError?.(error);
      }
    } finally {
      if (mountedRef.current && currentFetch === fetchCountRef.current) {
        setLoading(false);
        setIsFetching(false);
      }
    }
  }, [url, method, body, headers, enabled, cacheTime, retryCount, retryDelay, onSuccess, onError]);

  useEffect(() => {
    mountedRef.current = true;
    fetchData();

    return () => {
      mountedRef.current = false;
    };
  }, [fetchData]);

  return {
    data,
    loading,
    error,
    refetch: fetchData,
    isFetching,
  };
}

export function useApiMutation<T = any, B = any>(options?: Partial<UseApiOptions<T>>) {
  const [data, setData] = useState<T | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<Error | null>(null);

  const mutate = useCallback(async (body?: B) => {
    if (!options?.url) {
      throw new Error('URL is required for mutations');
    }

    setLoading(true);
    setError(null);

    try {
      const response = await fetch(options.url, {
        method: options.method || 'POST',
        headers: {
          'Content-Type': 'application/json',
          ...options.headers,
        },
        body: body ? JSON.stringify(body) : undefined,
      });

      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }

      const result = await response.json();
      setData(result);
      options.onSuccess?.(result);
      return result;
    } catch (err) {
      const error = err instanceof Error ? err : new Error('An error occurred');
      setError(error);
      options.onError?.(error);
      throw error;
    } finally {
      setLoading(false);
    }
  }, [options]);

  return {
    mutate,
    data,
    loading,
    error,
  };
}

export function useDebounce<T>(value: T, delay: number): T {
  const [debouncedValue, setDebouncedValue] = useState(value);

  useEffect(() => {
    const handler = setTimeout(() => {
      setDebouncedValue(value);
    }, delay);

    return () => {
      clearTimeout(handler);
    };
  }, [value, delay]);

  return debouncedValue;
}

export function useLocalStorage<T>(key: string, initialValue: T): [T, (value: T) => void] {
  const [storedValue, setStoredValue] = useState<T>(() => {
    try {
      const item = window.localStorage.getItem(key);
      return item ? JSON.parse(item) : initialValue;
    } catch (error) {
      console.error(`Error loading from localStorage key "${key}":`, error);
      return initialValue;
    }
  });

  const setValue = (value: T) => {
    try {
      setStoredValue(value);
      window.localStorage.setItem(key, JSON.stringify(value));
    } catch (error) {
      console.error(`Error saving to localStorage key "${key}":`, error);
    }
  };

  return [storedValue, setValue];
}
