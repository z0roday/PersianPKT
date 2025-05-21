use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub architecture: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub size: u64,
    pub installed_size: u64,
    pub maintainer: String,
    pub homepage: Option<String>,
    pub section: String,
    pub priority: String,
    pub filename: String,
    pub md5sum: String,
    pub sha256: String,
}

impl PackageInfo {
    pub fn new(
        name: String,
        version: String,
        architecture: String,
        description: String,
    ) -> Self {
        Self {
            name,
            version,
            architecture,
            description,
            dependencies: Vec::new(),
            size: 0,
            installed_size: 0,
            maintainer: String::new(),
            homepage: None,
            section: String::new(),
            priority: String::new(),
            filename: String::new(),
            md5sum: String::new(),
            sha256: String::new(),
        }
    }

    pub fn add_dependency(&mut self, dependency: String) {
        if !self.dependencies.contains(&dependency) {
            self.dependencies.push(dependency);
        }
    }

    pub fn set_size(&mut self, size: u64) {
        self.size = size;
    }

    pub fn set_installed_size(&mut self, installed_size: u64) {
        self.installed_size = installed_size;
    }

    pub fn set_maintainer(&mut self, maintainer: String) {
        self.maintainer = maintainer;
    }

    pub fn set_homepage(&mut self, homepage: String) {
        self.homepage = Some(homepage);
    }

    pub fn set_section(&mut self, section: String) {
        self.section = section;
    }

    pub fn set_priority(&mut self, priority: String) {
        self.priority = priority;
    }

    pub fn set_filename(&mut self, filename: String) {
        self.filename = filename;
    }

    pub fn set_md5sum(&mut self, md5sum: String) {
        self.md5sum = md5sum;
    }

    pub fn set_sha256(&mut self, sha256: String) {
        self.sha256 = sha256;
    }

    pub fn get_full_name(&self) -> String {
        format!("{}_{}", self.name, self.version)
    }

    pub fn get_dependencies(&self) -> &[String] {
        &self.dependencies
    }
} 