pub mod memory_cache;

use anyhow::Result;

use memory_cache::MemoryCache;

pub trait Cache {
    fn get(&self) -> Result<()>;
    fn put(&self) -> Result<()>;
}

pub enum Kind {
    MemoryCache,
}

// TODO: Return Cache trait
//pub fn new_cache(kind: Kind) -> Box<impl Cache> {
pub fn new_cache(kind: Kind) -> MemoryCache {
    /*match kind {
        MemoryCache => Box::new(MemoryCache::new()),
    }*/
    MemoryCache::new()
}
