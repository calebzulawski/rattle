use crate::error::Error;
use crate::file_cache::FileCache;
use crate::key::Key;
use std::collections::HashMap;
use std::time::SystemTime;

struct Output<T> {
    contents: T,
    build_iteration: u64,
    build_time: SystemTime,
}

trait Task<T> {
    fn requires() -> Vec<Key>;
    fn creates() -> Key;
    fn make(inputs: HashMap<Key, &T>, file_cache: FileCache) -> Result<T, Error>;
}
