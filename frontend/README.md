# ModelLink Desktop GUI

A modern desktop dashboard for managing ModelLink - the local proxy for AI coding tools.

## Features

- 📊 Real-time Dashboard with system metrics
- 🔗 Provider Management (OpenAI, Anthropic, DeepSeek, Gemini, Cohere)
- 📨 Request Inspector with detailed request/response viewing
- 📈 Prometheus-style Metrics Visualization
- ⚙️ Configurable Settings
- 🌙 Dark/Light Theme Support

## Tech Stack

- React 18 + TypeScript
- Tailwind CSS + DaisyUI
- Vite
- Recharts
- Zustand (State Management)
- React Query (Data Fetching)

## Getting Started

### Prerequisites

- Node.js 18+
- npm or yarn

### Installation

```bash
cd frontend
npm install
```

### Development

```bash
npm run dev
```

### Build

```bash
npm run build
```

## Project Structure

```
frontend/
├── src/
│   ├── components/       # React components
│   │   ├── Dashboard.tsx
│   │   ├── ProviderManager.tsx
│   │   ├── RequestInspector.tsx
│   │   ├── MetricsView.tsx
│   │   └── SettingsView.tsx
│   ├── App.tsx          # Main application
│   ├── main.tsx         # Entry point
│   └── index.css        # Global styles
├── package.json
├── vite.config.ts
├── tailwind.config.js
└── tsconfig.json
```

## Integration with Tauri

This frontend is designed to work with ModelLink's Tauri desktop application.

```bash
# Install Tauri CLI
npm install -g @tauri-apps/cli

# Initialize Tauri
npx tauri init

# Development
npm run tauri:dev

# Build
npm run tauri:build
```

## License

MIT
