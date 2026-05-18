# ModelLink 构建指南

## 构建要求

- Rust 1.70+
- Cargo

## 快速开始

### 1. 克隆仓库

```bash
git clone https://github.com/954510662-bot/ModelLink.git
cd ModelLink
```

### 2. 开发构建

```bash
cargo build
```

### 3. 发布构建

```bash
cargo build --release
```

### 4. 运行测试

```bash
cargo test
```

## 功能特性

### 默认功能

- 基础代理功能
- 配置热加载
- 健康检查
- Prometheus指标
- 配置版本迁移
- 自动备份
- Mock/离线模式
- Shell补全

### 可选功能

#### 在线更新功能

```bash
cargo build --release --features update
```

## 跨平台构建

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

## 使用方式

### 1. 生成配置文件

```bash
model-link config init
```

### 2. 启动服务

```bash
model-link start
```

### 3. 验证配置

```bash
model-link config validate
```

### 4. 诊断工具

```bash
model-link doctor
```

### 5. 查看版本

```bash
model-link version
```

### 6. Shell补全 (可选)

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

### 7. 在线更新 (需要启用 update feature)

```bash
# 检查更新
model-link update --check

# 更新到最新版本
model-link update

# 无需确认直接更新
model-link update --yes
```

## 配置说明

配置文件默认位置：

- Windows: `%APPDATA%\model-link\config.yaml`
- Linux/macOS: `~/.config/model-link/config.yaml`

查看配置文件位置：

```bash
model-link config path
```

## 项目结构

```
ModelLink/
├── src/
│   ├── bin/
│   │   └── model_link.rs       # 主程序入口
│   ├── audit.rs                # 审计日志
│   ├── backup.rs               # 配置备份
│   ├── cli.rs                  # 命令行接口
│   ├── config.rs               # 配置管理
│   ├── config_watcher.rs       # 配置热加载
│   ├── errors.rs               # 错误处理
│   ├── failover.rs             # 故障转移
│   ├── health.rs               # 健康检查
│   ├── lib.rs                  # 库入口
│   ├── metrics.rs              # Prometheus指标
│   ├── migration.rs            # 配置迁移
│   ├── mock.rs                 # Mock/离线模式
│   ├── models.rs               # 模型定义
│   ├── proxy.rs                # 代理转发
│   ├── server.rs               # 服务器
│   ├── stream.rs               # 流式处理
│   ├── translator.rs           # 参数转换
│   └── wizard.rs               # 配置向导
├── Cargo.toml
├── config-template.yaml
├── README.md
└── BUILD.md
```

## 开发

### 运行开发服务器

```bash
cargo run -- start
```

### 代码检查

```bash
cargo clippy
```

### 代码格式化

```bash
cargo fmt
```

## 发布流程

1. 更新版本号在 `Cargo.toml`
2. 创建 Git tag
3. 构建各平台二进制文件
4. 上传到 GitHub Releases

## 许可证

MIT License
