use anyhow::Result;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::fs;
use std::time::SystemTime;

use crate::package::{PackageInfo, PackageDependency};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub architecture: String,
    pub description: String,
    pub dependencies: Vec<PackageDependency>,
    pub conflicts: Vec<String>,
    pub provides: Vec<String>,
    pub replaces: Vec<String>,
    pub install_path: PathBuf,
    pub files: Vec<PathBuf>,
    pub install_date: SystemTime,
    pub size: u64,
    pub installed_size: u64,
    pub maintainer: String,
    pub homepage: Option<String>,
    pub section: String,
    pub priority: String,
}

impl Package {
    pub fn new(info: PackageInfo, install_path: PathBuf) -> Self {
        Self {
            name: info.name,
            version: info.version,
            architecture: info.architecture,
            description: info.description,
            dependencies: Vec::new(),
            conflicts: Vec::new(),
            provides: Vec::new(),
            replaces: Vec::new(),
            install_path,
            files: Vec::new(),
            install_date: SystemTime::now(),
            size: info.size,
            installed_size: info.installed_size,
            maintainer: info.maintainer,
            homepage: info.homepage,
            section: info.section,
            priority: info.priority,
        }
    }

    pub fn is_installed(&self) -> bool {
        self.install_path.exists()
    }

    pub fn add_file(&mut self, file: PathBuf) {
        if !self.files.contains(&file) {
            self.files.push(file);
        }
    }

    pub fn add_dependency(&mut self, dependency: PackageDependency) {
        if !self.dependencies.iter().any(|d| d.name == dependency.name) {
            self.dependencies.push(dependency);
        }
    }

    pub fn add_conflict(&mut self, conflict: String) {
        if !self.conflicts.contains(&conflict) {
            self.conflicts.push(conflict);
        }
    }

    pub fn add_provides(&mut self, provides: String) {
        if !self.provides.contains(&provides) {
            self.provides.push(provides);
        }
    }

    pub fn add_replaces(&mut self, replaces: String) {
        if !self.replaces.contains(&replaces) {
            self.replaces.push(replaces);
        }
    }

    pub fn get_files(&self) -> &[PathBuf] {
        &self.files
    }

    pub fn get_dependencies(&self) -> &[PackageDependency] {
        &self.dependencies
    }

    pub fn get_conflicts(&self) -> &[String] {
        &self.conflicts
    }

    pub fn get_provides(&self) -> &[String] {
        &self.provides
    }

    pub fn get_replaces(&self) -> &[String] {
        &self.replaces
    }

    pub fn save_to_file(&self, path: &PathBuf) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn load_from_file(path: &PathBuf) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let package = serde_json::from_str(&content)?;
        Ok(package)
    }
} 