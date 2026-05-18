# ModelLink Build Guide

## Requirements

- Rust 1.70+
- Cargo

## Quick Start

### 1. Clone repository

```bash
git clone https://github.com/954510662-bot/ModelLink.git
cd ModelLink
```

### 2. Development build

```bash
cargo build
```

### 3. Release build

```bash
cargo build --release
```

### 4. Run tests

```bash
cargo test
```

## Features

### Default features

- Core proxy functionality
- Hot config reloading
- Health checks
- Prometheus metrics
- Config version migration
- Automatic backups
- Mock/offline mode
- Shell completion

### Optional features

#### Auto-update feature

```bash
cargo build --release --features update
```

## Cross-platform builds

### Windows (x86_64)

```bash
cargo build --release --target x86_64-pc-windows-msvc
```

### Linux (x86_64)

```bash
cargo build --release --target x86_64-unknown-linux-gnu
```

### macOS (Intel)

```bash
cargo build --release --target x86_64-apple-darwin
```

### macOS (Apple Silicon)

```bash
cargo build --release --target aarch64-apple-darwin
```

## Usage

### 1. Generate config file

```bash
model-link config init
```

### 2. Start service

```bash
model-link start
```

### 3. Validate config

```bash
model-link config validate
```

### 4. Diagnostic tool

```bash
model-link doctor
```

### 5. Check version

```bash
model-link version
```

### 6. Shell completion (optional)

#### Bash

```bash
model-link completions --shell bash > ~/.bash_completion.d/model-link
```

#### Zsh

```bash
model-link completions --shell zsh > ~/.zfunc/_model-link
```

#### Fish

```bash
model-link completions --shell fish > ~/.config/fish/completions/model-link.fish
```

#### PowerShell

```powershell
model-link completions --shell powershell | Out-File -Encoding utf8 $PROFILE
```

### 7. Auto-update (requires update feature)

```bash
# Check for updates
model-link update --check

# Update to latest version
model-link update

# Update without confirmation
model-link update --yes
```

## Configuration

Default config file locations:

- Windows: `%APPDATA%\model-link\config.yaml`
- Linux/macOS: `~/.config/model-link/config.yaml`

View config file location:

```bash
model-link config path
```

## Project Structure

```
ModelLink/
├── src/
│   ├── bin/
│   │   └── model_link.rs       # Main entry point
│   ├── audit.rs                # Audit logging
│   ├── backup.rs               # Config backup
│   ├── cli.rs                  # Command-line interface
│   ├── config.rs               # Config management
│   ├── config_watcher.rs       # Hot config reloading
│   ├── errors.rs               # Error handling
│   ├── failover.rs             # Failover
│   ├── health.rs               # Health checks
│   ├── lib.rs                  # Library entry
│   ├── metrics.rs              # Prometheus metrics
│   ├── migration.rs            # Config migration
│   ├── mock.rs                 # Mock/offline mode
│   ├── models.rs               # Model definitions
│   ├── proxy.rs                # Proxy forwarding
│   ├── server.rs               # Server
│   ├── stream.rs               # Streaming
│   ├── translator.rs           # Parameter translation
│   └── wizard.rs               # Config wizard
├── Cargo.toml
├── config-template.yaml
├── README.md
└── BUILD.md
```

## Development

### Run development server

```bash
cargo run -- start
```

### Code linting

```bash
cargo clippy
```

### Code formatting

```bash
cargo fmt
```

## Release Process

1. Update version number in `Cargo.toml`
2. Create Git tag
3. Build binaries for all platforms
4. Upload to GitHub Releases

## License

MIT License
