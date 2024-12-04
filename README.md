# Memory Analysis Tool

[简体中文](./README.md) | English

A Windows process memory analysis tool based on memflow, providing memory read/write and pattern scanning capabilities.

## Features

### Process Management
- List all system processes
- Get process module information
- Process memory region analysis

### Memory Operations
- Read/Write process memory
- Memory region protection flag management
- Memory pattern scanning

### Tool Support
- Byte pattern matching
- Hexadecimal formatting
- Memory region information management

## Quick Start

### Requirements

- Rust 1.70+ 
- Windows system (for target analysis)
- Administrator privileges (for memory access)

### Dependencies

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

### Installation

1. Clone the project
```bash
git clone https://github.com/yourusername/memory-analysis-tool.git
cd memory-analysis-tool
```

2. Build the project
```bash
cargo build --release
```

3. Run examples
```bash
# List all processes
cargo run

# Scan specific process
cargo run -- -p notepad.exe
```

## Usage Guide

### Basic Usage
```rust
use memory_analysis_tool::{MemoryManager, services::ProcessService};

fn main() -> Result<()> {
    // Create memory manager
    let memory_manager = MemoryManager::new(connector)?;
    
    // Use process service
    let process_service = ProcessService::new(process);
    
    // Read memory
    let data = process_service.read_memory(address, size)?;
    
    Ok(())
}
```

### Memory Scanning
```rust
use memory_analysis_tool::utils::MemoryScanner;

// Create scan pattern
let pattern = MemoryScanner::create_pattern("48 8B 05 ?? ?? ?? ??")?;

// Scan memory
let matches = MemoryScanner::scan_pattern(&memory_data, &pattern);
```

## Project Structure

```
src/
├── main.rs          # Command line entry
├── lib.rs           # Library entry and core manager
├── models/          # Data models
│   ├── mod.rs
│   └── memory.rs    # Memory related structures
├── services/        # Business logic
│   ├── mod.rs
│   └── process_service.rs  # Process service
└── utils/           # Utility functions
    ├── mod.rs
    ├── scanner.rs   # Memory scanner
    └── memory.rs    # Memory utilities
```

## API Documentation

### MemoryManager
Memory manager providing Windows kernel interaction interface:
```rust
pub struct MemoryManager {
    kernel: Win32Kernel<impl PhysicalMemory>,
}
```

### ProcessService
Process service encapsulating process operations:
```rust
pub struct ProcessService {
    process: Process,
}
```

### MemoryScanner
Memory scanning tool supporting pattern matching:
```rust
pub struct MemoryScanner;
```

## Common Issues

1. Permission Errors
   - Ensure running with administrator privileges
   - Check process access permissions

2. Memory Access Failures
   - Confirm target process exists
   - Verify memory address validity

3. Pattern Matching Issues
   - Validate pattern format
   - Ensure appropriate memory range

## Contributing

1. Fork the project
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## License

MIT License

## Disclaimer

This tool is for educational and research purposes only. Users must comply with relevant laws and regulations and not use it for illegal purposes. The author is not responsible for any misuse.

## Author

记得晚安

## Contact

- GitHub: [记得晚安](https://github.com/AAASS554)
