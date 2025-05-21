use anyhow::Result;
use std::path::{Path, PathBuf};
use std::fs;
use std::time::{Duration, SystemTime};

pub struct CacheSystem {
    cache_dir: PathBuf,
    max_age: Duration,
}

impl CacheSystem {
    pub fn new(cache_dir: PathBuf) -> Self {
        Self {
            cache_dir,
            max_age: Duration::from_secs(60 * 60 * 24 * 7), // 7 days
        }
    }

    pub fn init(&self) -> Result<()> {
        if !self.cache_dir.exists() {
            fs::create_dir_all(&self.cache_dir)?;
        }
        Ok(())
    }

    pub fn get_package_path(&self, package_name: &str, version: &str) -> PathBuf {
        self.cache_dir.join(format!("{}_{}.pkg", package_name, version))
    }

    pub fn package_exists(&self, package_name: &str, version: &str) -> bool {
        self.get_package_path(package_name, version).exists()
    }

    pub fn store_package(&self, package_name: &str, version: &str, data: &[u8]) -> Result<()> {
        let path = self.get_package_path(package_name, version);
        fs::write(path, data)?;
        Ok(())
    }

    pub fn get_package(&self, package_name: &str, version: &str) -> Result<Option<Vec<u8>>> {
        let path = self.get_package_path(package_name, version);
        if path.exists() {
            let data = fs::read(path)?;
            Ok(Some(data))
        } else {
            Ok(None)
        }
    }

    pub fn clean_old_packages(&self) -> Result<usize> {
        let mut removed = 0;
        let now = SystemTime::now();
        
        for entry in fs::read_dir(&self.cache_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if let Ok(metadata) = fs::metadata(&path) {
                if let Ok(modified) = metadata.modified() {
                    if now.duration_since(modified)? > self.max_age {
                        fs::remove_file(path)?;
                        removed += 1;
                    }
                }
            }
        }
        
        Ok(removed)
    }

    pub fn clean_all(&self) -> Result<usize> {
        let mut removed = 0;
        
        for entry in fs::read_dir(&self.cache_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                fs::remove_file(path)?;
                removed += 1;
            }
        }
        
        Ok(removed)
    }

    pub fn get_cache_size(&self) -> Result<u64> {
        let mut total_size = 0;
        
        for entry in fs::read_dir(&self.cache_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if let Ok(metadata) = fs::metadata(&path) {
                if metadata.is_file() {
                    total_size += metadata.len();
                }
            }
        }
        
        Ok(total_size)
    }
} 