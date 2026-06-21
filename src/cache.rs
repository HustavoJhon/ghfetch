use serde::{de::DeserializeOwned, Serialize};
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

#[derive(Clone)]
pub struct Cache {
    dir: PathBuf,
}

impl Cache {
    pub fn new(dir: &str) -> Self {
        let d = PathBuf::from(dir);
        std::fs::create_dir_all(&d).ok();
        Self { dir: d }
    }

    fn path(&self, key: &str) -> PathBuf {
        let safe = key.replace(['/', '\\', ':', '?', '*', '"', '<', '>', '|'], "_");
        self.dir.join(format!("{}.json", safe))
    }

    pub fn get<T: DeserializeOwned>(&self, key: &str, ttl_hours: u64) -> Option<T> {
        let path = self.path(key);
        if !path.exists() {
            return None;
        }
        let meta = std::fs::metadata(&path).ok()?;
        let modified = meta.modified().ok()?;
        let age = SystemTime::now().duration_since(modified).unwrap_or(Duration::ZERO);
        if age > Duration::from_secs(ttl_hours * 3600) {
            return None;
        }
        let data = std::fs::read_to_string(&path).ok()?;
        serde_json::from_str(&data).ok()
    }

    pub fn set<T: Serialize>(&self, key: &str, value: &T) {
        let path = self.path(key);
        if let Ok(json) = serde_json::to_string(value) {
            std::fs::write(&path, json).ok();
        }
    }

    pub fn get_raw(&self, key: &str, ttl_hours: u64) -> Option<Vec<u8>> {
        let path = self.path(key);
        if !path.exists() {
            return None;
        }
        let meta = std::fs::metadata(&path).ok()?;
        let modified = meta.modified().ok()?;
        let age = SystemTime::now()
            .duration_since(modified)
            .unwrap_or(Duration::ZERO);
        if age > Duration::from_secs(ttl_hours * 3600) {
            return None;
        }
        std::fs::read(&path).ok()
    }

    pub fn set_raw(&self, key: &str, data: &[u8]) {
        let path = self.path(key);
        std::fs::write(&path, data).ok();
    }

    #[allow(dead_code)]
    pub fn raw_path(&self, key: &str) -> PathBuf {
        let safe = key.replace(['/', '\\', ':', '?', '*', '"', '<', '>', '|'], "_");
        self.dir.join(safe)
    }
}
