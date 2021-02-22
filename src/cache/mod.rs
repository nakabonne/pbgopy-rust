pub mod memory_cache;

use anyhow::Result;

pub trait Cache {
    fn get(&self) -> Result<()>;
    fn put(&self) -> Result<()>;
}
