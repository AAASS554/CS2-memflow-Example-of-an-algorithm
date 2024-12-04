# Memory Analysis Tool

[English](./README.md) | 简体中文

基于 memflow 的 Windows 进程内存分析工具，提供内存读写、模式扫描等功能。

## 功能特点

### 进程管理
- 列出系统所有进程
- 获取进程模块信息
- 进程内存区域分析

### 内存操作
- 读写进程内存
- 内存区域保护标志管理
- 内存模式扫描

### 工具支持
- 字节模式匹配
- 十六进制格式化
- 内存区域信息管理

## 快速开始

### 安装要求

- Rust 1.70+ 
- Windows 系统 (用于目标分析)
- 管理员权限 (用于内存访问)

### 依赖项

```toml
[dependencies]
memflow = "0.2.3"
memflow-win32 = "0.2"
memflow-native = { git = "https://github.com/memflow/memflow-native" }
dataview = "1.0.1"
log = "0.4.22"
simple_logger = "5.0.0"
anyhow = "1.0.93"
clap = { version = "4.5.21", features = ["derive"] }
```

### 安装步骤

1. 克隆项目

```bash
git clone https://github.com/yourusername/memory-analysis-tool.git
cd memory-analysis-tool
```

2. 构建项目

```bash
cargo build --release
```

3. 运行示例

```bash
# 列出所有进程
cargo run

# 扫描特定进程
cargo run -- -p notepad.exe
```

## 使用指南

### 基本使用

```rust
use memory_analysis_tool::{MemoryManager, services::ProcessService};

fn main() -> Result<()> {
    // 创建内存管理器
    let memory_manager = MemoryManager::new(connector)?;
    
    // 使用进程服务
    let process_service = ProcessService::new(process);
    
    // 读取内存
    let data = process_service.read_memory(address, size)?;
    
    Ok(())
}
```

### 内存扫描

```rust
use memory_analysis_tool::utils::MemoryScanner;

// 创建扫描模式
let pattern = MemoryScanner::create_pattern("48 8B 05 ?? ?? ?? ??")?;

// 扫描内存
let matches = MemoryScanner::scan_pattern(&memory_data, &pattern);
```

## 项目结构

```
src/
├── main.rs          # 命令行入口
├── lib.rs           # 库入口和核心管理器
├── models/          # 数据模型
│   ├── mod.rs
│   └── memory.rs    # 内存相关结构
├── services/        # 业务逻辑
│   ├── mod.rs
│   └── process_service.rs  # 进程服务
└── utils/           # 工具函数
    ├── mod.rs
    ├── scanner.rs   # 内存扫描器
    └── memory.rs    # 内存工具
```

## API 文档

### MemoryManager
内存管理器，提供与 Windows 内核的交互接口：

```rust
pub struct MemoryManager {
    kernel: Win32Kernel<impl PhysicalMemory>,
}
```

### ProcessService
进程服务，封装进程操作：

```rust
pub struct ProcessService {
    process: Process,
}
```

### MemoryScanner
内存扫描工具，支持模式匹配：

```rust
pub struct MemoryScanner;
```

## 常见问题

1. 权限错误
   - 确保以管理员身份运行
   - 检查进程访问权限

2. 内存访问失败
   - 确认目标进程存在
   - 检查内存地址有效性

3. 模式匹配问题
   - 验证模式格式正确
   - 确保内存范围合适

## 贡献指南

1. Fork 项目
2. 创建特性分支
3. 提交更改
4. 推送到分支
5. 创建 Pull Request

## 许可证

MIT License

## 免责声明

本工具仅用于教育和研究目的。使用者需要遵守相关法律法规，不得用于非法用途。作者不对任何滥用负责。
## 作者

记得晚安

## Contact

- GitHub: [记得晚安](https://github.com/AAASS554)