use std::path::{Path, PathBuf};
use dirs;

pub struct ConfigPaths {
    base_dir: PathBuf,
}

impl ConfigPaths {
    pub fn new() -> Self {
        let base_dir = if let Some(config_dir) = dirs::config_dir() {
            config_dir.join("persianpkt")
        } else {
            PathBuf::from("/etc/persianpkt")
        };
        
        Self { base_dir }
    }
    
    pub fn with_base_dir(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }
    
    pub fn base_dir(&self) -> PathBuf {
        self.base_dir.clone()
    }
    
    pub fn config_file(&self) -> PathBuf {
        self.base_dir.join("config.toml")
    }
    
    pub fn repositories_file(&self) -> PathBuf {
        self.base_dir.join("repositories.json")
    }
    
    pub fn mirrors_file(&self) -> PathBuf {
        self.base_dir.join("mirrors.json")
    }
    
    pub fn cache_dir(&self) -> PathBuf {
        if let Some(cache_dir) = dirs::cache_dir() {
            cache_dir.join("persianpkt")
        } else {
            PathBuf::from("/var/cache/persianpkt")
        }
    }
    
    pub fn packages_dir(&self) -> PathBuf {
        if let Some(data_dir) = dirs::data_dir() {
            data_dir.join("persianpkt").join("packages")
        } else {
            PathBuf::from("/var/lib/persianpkt/packages")
        }
    }
    
    pub fn keys_dir(&self) -> PathBuf {
        self.base_dir.join("keys")
    }
    
    pub fn logs_dir(&self) -> PathBuf {
        if let Some(data_dir) = dirs::data_dir() {
            data_dir.join("persianpkt").join("logs")
        } else {
            PathBuf::from("/var/log/persianpkt")
        }
    }
    
    pub fn temp_dir(&self) -> PathBuf {
        if let Some(cache_dir) = dirs::cache_dir() {
            cache_dir.join("persianpkt").join("temp")
        } else {
            PathBuf::from("/tmp/persianpkt")
        }
    }
    
    pub fn ensure_dirs_exist(&self) -> std::io::Result<()> {
        for dir in [
            self.base_dir(),
            self.cache_dir(),
            self.packages_dir(),
            self.keys_dir(),
            self.logs_dir(),
            self.temp_dir(),
        ].iter() {
            if !dir.exists() {
                std::fs::create_dir_all(dir)?;
            }
        }
        
        Ok(())
    }
} 