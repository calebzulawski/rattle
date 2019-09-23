use crate::error::Error;
use crate::key::Key;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct FileCache {
    root: PathBuf,
    keys: HashMap<Key, String>,
}

impl FileCache {
    pub fn reset(mut root: PathBuf) -> Result<Self, Error> {
        root.push(".file_cache");

        if root.exists() {
            std::fs::remove_dir_all(&root)?;
        }
        std::fs::create_dir(&root)?;

        Ok(Self {
            root: root,
            keys: HashMap::new(),
        })
    }

    pub fn new(mut root: PathBuf) -> Result<Self, Error> {
        root.push(".file_cache");
        if !root.is_dir() {
            Err(Error::FileCache(format!(
                "{} already exists and is not a directory",
                root.to_string_lossy()
            )))?;
        }

        let mut keys = HashMap::new();
        for key_entry in root.read_dir()? {
            let key_path = key_entry
                .map_err(|_| {
                    Error::FileCache("cache corrupted (could not read cache entry)".to_string())
                })?
                .path();
            let key_str = key_path.to_str().ok_or(Error::FileCache(
                "cache corrupted (could not parse directory name)".to_string(),
            ))?;
            if !key_path.is_dir() {
                Err(Error::FileCache(format!(
                    "cache corrupted (unexpected file {})",
                    key_str
                )))?;
            }
            keys.insert(
                Key::from_base64(key_str).ok_or(Error::FileCache(format!(
                    "cache corrupted (invalid key path {})",
                    key_str
                )))?,
                key_str.to_string(),
            );
        }

        Ok(Self {
            root: root,
            keys: keys,
        })
    }

    pub fn contains(&self, key: &Key) -> bool {
        self.keys.contains_key(key)
    }

    pub fn get_key_dir(&self, key: &Key) -> Option<PathBuf> {
        self.keys.get(key).map(|dir| {
            let mut path = self.root.clone();
            path.push(dir);
            path
        })
    }

    pub fn clean_key_dir(&mut self, key: &Key) -> Result<(), Error> {
        if self.contains(key) {
            self.remove_key_dir(key)?;
            self.create_key_dir(key)?;
        }
        Ok(())
    }

    pub fn remove_key_dir(&mut self, key: &Key) -> Result<(), Error> {
        if let Some(dir) = self.get_key_dir(key) {
            std::fs::remove_dir_all(&dir)?;
            self.keys.remove(key);
        }
        Ok(())
    }

    pub fn create_key_dir(&mut self, key: &Key) -> Result<(), Error> {
        let key_base64 = key.as_base64();
        let mut path = self.root.clone();
        path.push(&key_base64);
        self.keys.insert(key.clone(), key_base64);
        std::fs::create_dir(&path)?;
        Ok(())
    }
}
