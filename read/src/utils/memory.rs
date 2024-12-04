use std::fmt;

#[derive(Debug, Clone)]
pub struct MemoryRegionInfo {
    pub start: u64,
    pub size: usize,
    pub name: String,
    pub protection: Protection,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Protection {
    Read,
    Write,
    Execute,
    ReadWrite,
    ReadExecute,
    ReadWriteExecute,
    None,
}

impl fmt::Display for Protection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Protection::Read => write!(f, "R--"),
            Protection::Write => write!(f, "-W-"),
            Protection::Execute => write!(f, "--X"),
            Protection::ReadWrite => write!(f, "RW-"),
            Protection::ReadExecute => write!(f, "R-X"),
            Protection::ReadWriteExecute => write!(f, "RWX"),
            Protection::None => write!(f, "---"),
        }
    }
}

pub fn format_bytes(bytes: &[u8]) -> String {
    bytes.iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(" ")
} 