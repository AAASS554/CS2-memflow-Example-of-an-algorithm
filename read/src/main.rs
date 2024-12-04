use log::*;
use memflow::prelude::v1::*;
use memflow_win32::prelude::v1::*;
use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 进程名称（可选）
    #[arg(short, long)]
    process: Option<String>,

    /// 连接器类型
    #[arg(short, long, default_value = "native")]
    connector: String,
}

fn main() -> Result<()> {
    // 初始化日志
    simple_logger::SimpleLogger::new().init()?;

    // 解析命令行参数
    let args = Args::parse();

    // 创建连接器配置
    let mut inventory = ConnectorInventory::try_new()?;
    
    // 创建 native 连接器
    let connector = unsafe {
        inventory.create_connector(&args.connector, &ConnectorArgs::new())?
    };

    // 创建 Windows 内核接口
    let mut kernel = unsafe {
        Win32Kernel::builder(connector)
            .build()?
    };

    // 如果指定了进程名，则扫描特定进程
    if let Some(process_name) = args.process {
        scan_specific_process(&mut kernel, &process_name)?;
    } else {
        // 否则列出所有进程
        list_all_processes(&mut kernel)?;
    }

    Ok(())
}

fn list_all_processes(kernel: &mut Win32Kernel<impl PhysicalMemory>) -> Result<()> {
    info!("Listing all processes...");
    let process_list = kernel.process_info_list()?;
    
    for process in process_list {
        info!(
            "Process: {} (PID: {}) Base: {:x}",
            process.name,
            process.pid,
            process.base.as_u64()
        );
    }
    Ok(())
}

fn scan_specific_process(kernel: &mut Win32Kernel<impl PhysicalMemory>, process_name: &str) -> Result<()> {
    info!("Scanning process: {}", process_name);
    
    let process = kernel
        .process_info_list()?
        .into_iter()
        .find(|p| p.name.to_lowercase() == process_name.to_lowercase())
        .ok_or_else(|| anyhow::anyhow!("Process not found: {}", process_name))?;

    info!(
        "Found process: {} (PID: {}) Base: {:x}",
        process.name,
        process.pid,
        process.base.as_u64()
    );

    // 获取进程句柄
    let mut proc_handle = process.into_process(kernel)?;
    
    // 获取模块信息
    let modules = proc_handle.module_list()?;
    info!("Modules for process {}:", process_name);
    for module in modules {
        info!(
            "Module: {} Base: {:x} Size: {:x}",
            module.name,
            module.base.as_u64(),
            module.size,
        );
    }

    Ok(())
} 