use anyhow::Result;
use log::*;
use memflow::prelude::v1::*;

pub struct MemoryScanner;

impl MemoryScanner {
    pub fn scan_pattern(buffer: &[u8], pattern: &[u8]) -> Vec<usize> {
        let mut matches = Vec::new();
        if pattern.is_empty() || pattern.len() > buffer.len() {
            return matches;
        }

        for (i, window) in buffer.windows(pattern.len()).enumerate() {
            if window == pattern {
                matches.push(i);
            }
        }
        matches
    }

    pub fn create_pattern(pattern: &str) -> Result<Vec<u8>> {
        let pattern = pattern.replace(" ", "");
        let mut bytes = Vec::new();
        
        let mut chars = pattern.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                '?' => {
                    bytes.push(0);
                }
                '0'..='9' | 'A'..='F' | 'a'..='f' => {
                    if let Some(next) = chars.next() {
                        let hex = format!("{}{}", c, next);
                        let byte = u8::from_str_radix(&hex, 16)?;
                        bytes.push(byte);
                    }
                }
                _ => return Err(anyhow::anyhow!("Invalid pattern character: {}", c)),
            }
        }
        Ok(bytes)
    }
} 