# ModelLink Windows 版本构建指南

## 方案一：在 WSL 中交叉编译（推荐）

### 1. 安装交叉编译工具链

在 WSL 中运行：

```bash
# 安装 MinGW-w64
sudo apt update
sudo apt install -y gcc-mingw-w64-x86-64

# 添加 Windows 目标
rustup target add x86_64-pc-windows-gnu
```

### 2. 配置 Cargo

在项目根目录创建 `.cargo/config.toml`：

```toml
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
ar = "x86_64-w64-mingw32-gcc-ar"
```

### 3. 构建 Windows 版本

```bash
cd /mnt/d/WSL-Windows.Projects/ModelLink
cargo build --release --target x86_64-pc-windows-gnu
```

构建完成后，二进制文件位于：
`target/x86_64-pc-windows-gnu/release/model-link.exe`

---

## 方案二：在 Windows 原生环境中构建

### 1. 安装 Rust

访问 https://rustup.rs/ 下载并安装 Rust for Windows

### 2. 安装 Visual Studio Build Tools

下载并安装：https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022

选择 "使用 C++ 的桌面开发" 工作负载

### 3. 构建

在 PowerShell 中运行：

```powershell
cd D:\WSL-Windows.Projects\ModelLink
cargo build --release
```

### 4. 如果遇到代理问题

```powershell
# 临时禁用代理
$env:HTTP_PROXY=""
$env:HTTPS_PROXY=""
cargo build --release
```

---

## 方案三：使用 GitHub Actions 自动构建（最简单）

### 创建 GitHub Actions 工作流

在项目中创建 `.github/workflows/build.yml`：

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

### 触发构建

1. 推送到 GitHub
2. 创建一个 tag：`git tag v0.1.0 && git push origin v0.1.0`
3. GitHub Actions 会自动构建所有平台版本
4. 在 Actions 页面下载构建好的二进制文件

---

## 打包发布

### Windows 版本打包

1. 复制二进制文件
2. 创建 ZIP 压缩包

```powershell
Compress-Archive -Path target\release\model-link.exe -DestinationPath model-link-windows-x64.zip
```

### 创建发布说明模板

```markdown
# ModelLink v0.1.0

## 下载

- [Windows x64](model-link-windows-x64.zip)
- [Linux x64](model-link-linux-x64.tar.gz)
- [macOS x64](model-link-macos-x64.tar.gz)

## 快速开始

### Windows

```powershell
# 解压
Expand-Archive model-link-windows-x64.zip -DestinationPath .\model-link

# 生成配置
.\model-link\model-link.exe config init

# 启动服务
.\model-link\model-link.exe start
```

## 功能特性

- ✅ OpenAI/Anthropic 协议转换
- ✅ 配置热加载
- ✅ 健康检查
- ✅ Prometheus 指标
- ✅ 故障转移
- ✅ 配置备份与迁移
- ✅ Mock/离线模式
- ✅ Shell 补全
- ✅ 在线更新 (可选)
```

---

## 验证构建

### 基本功能测试

```powershell
# 查看版本
.\model-link.exe version

# 诊断检查
.\model-link.exe doctor

# 生成配置
.\model-link.exe config init

# 验证配置
.\model-link.exe config validate
```

---

## 常见问题

### Q: 编译时遇到 proc-macro 相关错误？
A: 尝试更新 Rust：`rustup update`，然后清理缓存：`cargo clean`

### Q: 网络问题导致依赖下载失败？
A: 配置 Cargo 使用国内镜像源，在 `~/.cargo/config.toml` 中添加：

```toml
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "sparse+https://mirrors.ustc.edu.cn/crates.io-index/"
```

### Q: 缺少 Visual Studio Build Tools？
A: 下载并安装：https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022
