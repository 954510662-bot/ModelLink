# ModelLink Windows Build Guide

## Option 1: Cross-compile in WSL (Recommended)

### 1. Install cross-compilation toolchain

Run in WSL:

```bash
# Install MinGW-w64
sudo apt update
sudo apt install -y gcc-mingw-w64-x86-64

# Add Windows target
rustup target add x86_64-pc-windows-gnu
```

### 2. Configure Cargo

Create `.cargo/config.toml` in project root:

```toml
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
ar = "x86_64-w64-mingw32-gcc-ar"
```

### 3. Build Windows version

```bash
cd /mnt/d/WSL-Windows.Projects/ModelLink
cargo build --release --target x86_64-pc-windows-gnu
```

After build, binary is located at:
`target/x86_64-pc-windows-gnu/release/model-link.exe`

---

## Option 2: Build natively on Windows

### 1. Install Rust

Visit https://rustup.rs/ to download and install Rust for Windows

### 2. Install Visual Studio Build Tools

Download and install: https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022

Select "Desktop development with C++" workload

### 3. Build

Run in PowerShell:

```powershell
cd D:\WSL-Windows.Projects\ModelLink
cargo build --release
```

### 4. If encountering proxy issues

```powershell
# Temporarily disable proxy
$env:HTTP_PROXY=""
$env:HTTPS_PROXY=""
cargo build --release
```

---

## Option 3: GitHub Actions Auto-build (Easiest)

### Create GitHub Actions workflow

Create `.github/workflows/build.yml` in project:

```yaml
name: Build ModelLink

on:
  push:
    tags:
      - 'v*'
  workflow_dispatch:

jobs:
  build:
    name: Build on ${{ matrix.os }}
    runs-on: ${{ matrix.os }}
    strategy:
      fail-fast: false
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
            artifact_name: model-link-linux
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            artifact_name: model-link-windows.exe
          - os: macos-latest
            target: x86_64-apple-darwin
            artifact_name: model-link-macos

    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}

      - name: Build
        run: cargo build --release --target ${{ matrix.target }}

      - name: Upload artifact
        uses: actions/upload-artifact@v4
        with:
          name: ${{ matrix.artifact_name }}
          path: target/${{ matrix.target }}/release/model-link${{ matrix.os == 'windows-latest' && '.exe' || '' }}
```

### Trigger build

1. Push to GitHub
2. Create a tag: `git tag v0.1.0 && git push origin v0.1.0`
3. GitHub Actions will automatically build all platform versions
4. Download built binaries from Actions page

---

## Packaging & Release

### Windows version packaging

1. Copy binary file
2. Create ZIP archive

```powershell
Compress-Archive -Path target\release\model-link.exe -DestinationPath model-link-windows-x64.zip
```

### Create release notes template

```markdown
# ModelLink v0.1.0

## Downloads

- [Windows x64](model-link-windows-x64.zip)
- [Linux x64](model-link-linux-x64.tar.gz)
- [macOS x64](model-link-macos-x64.tar.gz)

## Quick Start

### Windows

```powershell
# Extract
Expand-Archive model-link-windows-x64.zip -DestinationPath .\model-link

# Generate config
.\model-link\model-link.exe config init

# Start service
.\model-link\model-link.exe start
```

## Features

- ✅ OpenAI/Anthropic protocol conversion
- ✅ Hot config reloading
- ✅ Health checks
- ✅ Prometheus metrics
- ✅ Failover
- ✅ Config backup & migration
- ✅ Mock/offline mode
- ✅ Shell completion
- ✅ Auto-update (optional)
```

---

## Verify Build

### Basic functionality test

```powershell
# Check version
.\model-link.exe version

# Run diagnostics
.\model-link.exe doctor

# Generate config
.\model-link.exe config init

# Validate config
.\model-link.exe config validate
```

---

## FAQ

### Q: Encountering proc-macro related errors during compilation?
A: Try updating Rust: `rustup update`, then clean cache: `cargo clean`

### Q: Network issues causing dependency download failures?
A: Configure Cargo to use domestic mirror, add to `~/.cargo/config.toml`:

```toml
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "sparse+https://mirrors.ustc.edu.cn/crates.io-index/"
```

### Q: Missing Visual Studio Build Tools?
A: Download and install: https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022
