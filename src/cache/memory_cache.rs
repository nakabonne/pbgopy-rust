use super::Cache;

use anyhow::Result;

pub struct MemoryCache {}

impl MemoryCache {
    pub fn new() -> MemoryCache {
        MemoryCache {}
    }
}

impl Cache for MemoryCache {
    fn get(&self) -> Result<()> {
        Ok(())
    }
    fn put(&self) -> Result<()> {
        Ok(())
    }
}
