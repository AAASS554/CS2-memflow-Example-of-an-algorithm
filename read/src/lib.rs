pub mod models;
pub mod services;
pub mod utils;
pub mod config;

use anyhow::Result;
use log::*;
use memflow::prelude::v1::*;
use memflow_win32::prelude::v1::*;

pub struct MemoryManager {
    kernel: Win32Kernel<impl PhysicalMemory>,
}

impl MemoryManager {
    pub fn new(connector: impl PhysicalMemory + 'static) -> Result<Self> {
        let kernel = unsafe {
            Win32Kernel::builder(connector)
                .build()?
        };
        Ok(Self { kernel })
    }

    pub fn get_kernel_mut(&mut self) -> &mut Win32Kernel<impl PhysicalMemory> {
        &mut self.kernel
    }
}

// 库的公共 API
pub fn run() {
    println!("Library is running!");
} 