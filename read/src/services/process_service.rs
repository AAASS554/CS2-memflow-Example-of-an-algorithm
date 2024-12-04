use anyhow::Result;
use log::*;
use memflow::prelude::v1::*;
use memflow_win32::prelude::v1::*;

pub struct ProcessService {
    process: Process,
}

impl ProcessService {
    pub fn new(process: Process) -> Self {
        Self { process }
    }

    pub fn get_modules(&mut self) -> Result<Vec<ModuleInfo>> {
        Ok(self.process.module_list()?)
    }

    pub fn get_memory_regions(&mut self) -> Result<Vec<MemoryRegion>> {
        Ok(self.process.memory_regions()?)
    }

    pub fn read_memory(&mut self, address: u64, size: usize) -> Result<Vec<u8>> {
        let mut buffer = vec![0u8; size];
        self.process.read_raw_into(address.into(), &mut buffer)?;
        Ok(buffer)
    }

    pub fn write_memory(&mut self, address: u64, data: &[u8]) -> Result<()> {
        self.process.write_raw(address.into(), data)?;
        Ok(())
    }
} 