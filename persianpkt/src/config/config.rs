use anyhow::Result;
use serde::{Serialize, Deserialize};
use std::path::{Path, PathBuf};
use std::fs;

use crate::config::paths::ConfigPaths;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub repositories_file: PathBuf,
    pub cache_dir: PathBuf,
    pub packages_dir: PathBuf,
    pub keys_dir: PathBuf,
    pub default_mirrors: Vec<String>,
    pub architecture: String,
    pub mirrors_file: PathBuf,
    pub verbose: bool,
    pub auto_clean: bool,
    pub max_cache_size: u64,
    pub default_yes: bool,
}

impl Default for Config {
    fn default() -> Self {
        let paths = ConfigPaths::new();
        
        Self {
            repositories_file: paths.repositories_file(),
            cache_dir: paths.cache_dir(),
            packages_dir: paths.packages_dir(),
            keys_dir: paths.keys_dir(),
            default_mirrors: vec![
                "https://mirror.iran-server.com/debian/".to_string(),
                "https://debian.iranserver.com/debian/".to_string(),
                "https://mirror.arvancloud.com/debian/".to_string(),
            ],
            architecture: "amd64".to_string(),
            mirrors_file: paths.mirrors_file(),
            verbose: false,
            auto_clean: true,
            max_cache_size: 1024 * 1024 * 1024, // 1 GB
            default_yes: false,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn load(config_path: &Path) -> Result<Self> {
        if !config_path.exists() {
            let config = Self::default();
            config.save(config_path)?;
            return Ok(config);
        }
        
        let content = fs::read_to_string(config_path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
    
    pub fn save(&self, config_path: &Path) -> Result<()> {
        if let Some(parent) = config_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }
        
        let content = toml::to_string_pretty(self)?;
        fs::write(config_path, content)?;
        Ok(())
    }
    
    pub fn with_architecture(mut self, architecture: String) -> Self {
        self.architecture = architecture;
        self
    }
    
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }
    
    pub fn with_auto_clean(mut self, auto_clean: bool) -> Self {
        self.auto_clean = auto_clean;
        self
    }
    
    pub fn with_max_cache_size(mut self, max_cache_size: u64) -> Self {
        self.max_cache_size = max_cache_size;
        self
    }
    
    pub fn with_default_yes(mut self, default_yes: bool) -> Self {
        self.default_yes = default_yes;
        self
    }
    
    pub fn add_default_mirror(&mut self, mirror: String) {
        if !self.default_mirrors.contains(&mirror) {
            self.default_mirrors.push(mirror);
        }
    }
    
    pub fn remove_default_mirror(&mut self, mirror: &str) {
        self.default_mirrors.retain(|m| m != mirror);
    }
} 