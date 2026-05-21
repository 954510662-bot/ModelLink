# ModelLink Development Guide

## Overview

This guide provides detailed information for developers contributing to ModelLink.

## System Requirements

- Rust 1.75.0 or later
- Cargo package manager
- Git

## Getting Started

### Clone the Repository

```bash
git clone https://github.com/954510662-bot/ModelLink.git
cd ModelLink
```

### Build the Project

```bash
# Build in debug mode
cargo build

# Build in release mode
cargo build --release
```

### Run the Tests

```bash
# Run all tests
cargo test --all

# Run tests with verbose output
cargo test --all --verbose

# Run tests for a specific module
cargo test --package model-link --test <test_name>
```

### Run the Application

```bash
# Start the server with default config
cargo run -- start

# Start with custom config
cargo run -- start --config /path/to/config.yaml

# Generate shell completions
cargo run -- completions --shell bash
```

## Project Structure

```
ModelLink/
├── src/
│   ├── bin/
│   │   └── model_link.rs       # Main entry point
│   ├── audit.rs               # Audit logging
│   ├── backup.rs              # Config backup
│   ├── cli.rs                 # Command line interface
│   ├── config.rs              # Configuration management
│   ├── config_watcher.rs      # Hot reload
│   ├── errors.rs              # Error handling
│   ├── failover.rs            # Failover mechanism
│   ├── health.rs              # Health checks
│   ├── http_client.rs         # HTTP client pool
│   ├── metrics.rs             # Prometheus metrics
│   ├── migration.rs           # Config migrations
│   ├── mock.rs                # Mock mode
│   ├── models.rs              # Data models
│   ├── proxy.rs               # Proxy router
│   ├── rate_limit.rs          # Rate limiting
│   ├── server.rs              # Server initialization
│   ├── stream.rs              # Streaming handling
│   ├── translator.rs          # Parameter translation
│   ├── validation.rs          # Input validation
│   ├── wizard.rs              # Config wizard
│   └── lib.rs                 # Library exports
├── assets/                    # Static assets
├── tests/                     # Integration tests
├── Cargo.toml                 # Dependencies
├── config-template.yaml       # Config template
├── README.md                  # Main documentation
├── API.md                     # API documentation
└── BUILD.md                   # Build instructions
```

## Module Responsibilities

### Core Modules

| Module | Responsibility |
|--------|----------------|
| `config` | Load and manage configuration |
| `proxy` | HTTP request routing and handling |
| `server` | Server initialization and shutdown |
| `stream` | Streaming request handling |
| `translator` | Protocol and parameter translation |

### Security Modules

| Module | Responsibility |
|--------|----------------|
| `validation` | Input validation and sanitization |
| `rate_limit` | Rate limiting middleware |
| `audit` | Audit logging |

### Reliability Modules

| Module | Responsibility |
|--------|----------------|
| `failover` | Provider failover and health checks |
| `backup` | Config backup and restore |
| `migration` | Config version migrations |

### Performance Modules

| Module | Responsibility |
|--------|----------------|
| `http_client` | HTTP client connection pool |
| `metrics` | Prometheus metrics collection |

## Development Workflow

### Branching Strategy

- `main` - Stable production branch
- `develop` - Development branch
- `feature/*` - Feature branches
- `bugfix/*` - Bug fix branches

### Code Style

- Follow Rust style guidelines (run `cargo fmt`)
- Use `cargo clippy` for linting
- Add documentation comments for public APIs
- Include unit tests for new functionality

### Commit Messages

Use conventional commit format:
- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation
- `refactor:` Code refactoring
- `test:` Test updates
- `chore:` Build/CI updates

## Testing Guidelines

### Unit Tests

- Add tests for all public functions
- Test edge cases and error conditions
- Use `#[cfg(test)]` modules

### Integration Tests

- Place in `tests/` directory
- Test end-to-end scenarios
- Use mockito for HTTP mocking

### Test Coverage

Aim for at least 80% test coverage. Run:
```bash
cargo tarpaulin --out Html
```

## Debugging

### Enable Logging

Set `RUST_LOG` environment variable:

```bash
RUST_LOG=debug cargo run -- start
```

### Debug with VS Code

Add `.vscode/launch.json`:
```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug",
      "program": "${workspaceFolder}/target/debug/model-link",
      "args": ["start"],
      "cwd": "${workspaceFolder}"
    }
  ]
}
```

## Configuration

### Default Configuration

```yaml
server:
  host: 0.0.0.0
  port: 9191

providers:
  default:
    base_url: "https://api.openai.com/v1"
    api_key: "your-api-key"

mappings:
  gpt-4: "gpt-4"
  claude-3-opus: "claude-3-opus"

logging:
  level: "info"

security:
  audit_enabled: true
  masking_enabled: true

rate_limit:
  enabled: true
  requests_per_second: 10
  burst_limit: 50
```

## Adding a New Provider

1. Add provider configuration schema in `config.rs`
2. Implement provider-specific translation in `translator.rs`
3. Add model capabilities in `config.rs` `ModelCapabilityDB`
4. Add tests for the new provider
5. Update documentation

## Adding a New API Endpoint

1. Add route in `proxy.rs` `create_router()`
2. Implement handler function
3. Add input validation in `validation.rs`
4. Add tests
5. Update `API.md` documentation

## Release Process

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Run all tests
4. Create a tag: `git tag vX.Y.Z`
5. Push tag: `git push origin vX.Y.Z`
6. GitHub Actions will build and publish

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make changes and add tests
4. Submit a Pull Request
5. Address review comments
6. Merge when approved

## License

ModelLink is licensed under the MIT License. See `LICENSE` file for details.

## Resources

- [Rust Documentation](https://doc.rust-lang.org/)
- [Axum Documentation](https://docs.rs/axum/latest/axum/)
- [Reqwest Documentation](https://docs.rs/reqwest/latest/reqwest/)
- [Serde Documentation](https://docs.rs/serde/latest/serde/)

## Support

For questions and support, open an issue on GitHub.