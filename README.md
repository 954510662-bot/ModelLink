# ModelLink

A local proxy that allows AI coding tools to transparently use any third-party model.

## Features

- 🔄 **Protocol Disguise & Forwarding Engine** - Transparently convert and forward OpenAI-compatible API requests to other providers
- 📁 **Configuration Driven & Hot Reloading** - No service restart needed for configuration changes
- 🔗 **Parameter Adaptive Translation** - Safely handle unsupported parameters
- 📊 **Prometheus Metrics** - Real-time monitoring of request counts, errors, latency
- 📝 **Audit Logging** - Comprehensive request logging with sensitive data masking
- 🔄 **Failover Mechanism** - Automatic health checking and provider switchover
- 📦 **Configuration Versioning** - Automatic migrations when upgrading
- 💾 **Automatic Backups** - Safe configuration management
- 🎭 **Mock/Offline Mode** - Development and testing support
- 🐚 **Shell Completion** - Built-in shell completion support

## Quick Start

### Installation

From source:
```bash
git clone https://github.com/954510662-bot/ModelLink.git
cd ModelLink
cargo install --path .
```

### Configuration

Initialize default configuration:
```bash
model-link config init
```

This will create a configuration file at `~/.config/model-link/config.yaml`.

### Usage

Start the proxy:
```bash
model-link start
```

With custom port:
```bash
model-link start --port 8080
```

## Features in Detail

### Shell Completion

Generate shell completion scripts:

```bash
# Bash
model-link completions --shell bash

# Zsh
model-link completions --shell zsh

# Fish
model-link completions --shell fish

# PowerShell
model-link completions --shell powershell
```

### Health Check

Check service health:
```bash
curl http://localhost:9191/health
curl http://localhost:9191/ready
```

### Metrics

Get Prometheus metrics:
```bash
curl http://localhost:9191/metrics
```

## Configuration

```yaml
server:
  host: 0.0.0.0
  port: 9191

providers:
  default:
    base_url: "https://api.openai.com/v1"
    api_key: "your-api-key-here"
    capabilities:
      streaming: true
      supports_function_calling: true
      supports_temperature: true

mappings:
  gpt-4: "gpt-4"
  gpt-3.5-turbo: "gpt-3.5-turbo"

logging:
  level: "info"

security:
  audit_enabled: false
  masking_enabled: true

failover:
  enabled: false
  health_check_interval_secs: 30
```

## Development

Run tests:
```bash
cargo test
```

Build release:
```bash
cargo build --release
```

## License

MIT License - see LICENSE file for details.

## Contributing

Contributions welcome! Please feel free to submit a Pull Request.
