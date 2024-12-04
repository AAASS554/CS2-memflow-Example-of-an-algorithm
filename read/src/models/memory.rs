use memflow::prelude::v1::*;
use memflow_win32::prelude::v1::*;
use anyhow::{Result, anyhow};
use log::*;

pub struct MemoryScanner {
    kernel: Win32Kernel<impl PhysicalMemory>,
}

impl MemoryScanner {
    pub fn new(connector: impl PhysicalMemory + 'static) -> Result<Self> {
        let kernel = unsafe {
            Win32Kernel::builder(connector)
                .build()
                .map_err(|e| anyhow!("Failed to create kernel: {}", e))?
        };
        
        Ok(Self { kernel })
    }

    pub fn scan_process(&mut self, process_name: &str) -> Result<ProcessInstance> {
        let process = self.kernel
            .process_info_list()?
            .into_iter()
            .find(|p| p.name.to_lowercase() == process_name.to_lowercase())
            .ok_or_else(|| anyhow!("Process not found: {}", process_name))?;

        info!("Found process {} with PID {}", process_name, process.pid);
        
        Ok(process.into_process(&mut self.kernel)?)
    }

    pub fn read_memory(&mut self, process: &mut ProcessInstance, address: u64, size: usize) -> Result<Vec<u8>> {
        let mut buffer = vec![0u8; size];
        process.read_raw_into(address.into(), &mut buffer)?;
        Ok(buffer)
    }

    pub fn write_memory(&mut self, process: &mut ProcessInstance, address: u64, data: &[u8]) -> Result<()> {
        process.write_raw(address.into(), data)?;
        Ok(())
    }

    pub fn find_pattern(&mut self, process: &mut ProcessInstance, pattern: &[u8], start: u64, end: u64) -> Result<Vec<u64>> {
        let mut matches = Vec::new();
        let chunk_size = 0x1000;
        
        for addr in (start..end).step_by(chunk_size) {
            let size = std::cmp::min(chunk_size, (end - addr) as usize);
            if let Ok(buffer) = self.read_memory(process, addr, size) {
                if let Some(offset) = find_pattern_in_buffer(&buffer, pattern) {
                    matches.push(addr + offset as u64);
                }
            }
        }
        
        Ok(matches)
    }
}

fn find_pattern_in_buffer(buffer: &[u8], pattern: &[u8]) -> Option<usize> {
    buffer.windows(pattern.len())
        .position(|window| window == pattern)
} 