use anyhow::Result;
use reqwest::blocking::Client;
use std::collections::HashMap;
use std::time::Duration;
use url::Url;

use crate::repository::Repository;
use crate::package::PackageInfo;

pub struct RepositorySource {
    client: Client,
    timeout: Duration,
}

impl RepositorySource {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .unwrap_or_default(),
            timeout: Duration::from_secs(30),
        }
    }

    pub fn fetch_package_list(&self, repository: &Repository, component: &str, arch: &str) -> Result<Vec<PackageInfo>> {
        let url = repository.get_package_list_url(component, arch)?;
        let response = self.client.get(url).send()?;
        let content = response.text()?;
        
        let packages = self.parse_packages_file(&content)?;
        Ok(packages)
    }

    pub fn fetch_release_info(&self, repository: &Repository) -> Result<HashMap<String, String>> {
        let url = repository.get_release_url()?;
        let response = self.client.get(url).send()?;
        let content = response.text()?;
        
        let release_info = self.parse_release_file(&content)?;
        Ok(release_info)
    }

    pub fn download_package(&self, url: &Url) -> Result<Vec<u8>> {
        let response = self.client.get(url.as_str()).send()?;
        let content = response.bytes()?.to_vec();
        Ok(content)
    }

    fn parse_packages_file(&self, content: &str) -> Result<Vec<PackageInfo>> {
        let mut packages = Vec::new();
        let mut current_package = HashMap::new();
        
        for line in content.lines() {
            if line.is_empty() {
                if !current_package.is_empty() {
                    if let Some(package_info) = self.create_package_info(&current_package) {
                        packages.push(package_info);
                    }
                    current_package = HashMap::new();
                }
                continue;
            }
            
            if line.starts_with(' ') {
                continue;
            }
            
            if let Some(pos) = line.find(':') {
                let key = line[..pos].trim().to_string();
                let value = line[pos+1..].trim().to_string();
                current_package.insert(key, value);
            }
        }
        
        if !current_package.is_empty() {
            if let Some(package_info) = self.create_package_info(&current_package) {
                packages.push(package_info);
            }
        }
        
        Ok(packages)
    }

    fn create_package_info(&self, fields: &HashMap<String, String>) -> Option<PackageInfo> {
        let name = fields.get("Package")?.to_string();
        let version = fields.get("Version")?.to_string();
        let architecture = fields.get("Architecture")?.to_string();
        let description = fields.get("Description").cloned().unwrap_or_default();
        
        let mut info = PackageInfo {
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
        };
        
        if let Some(size_str) = fields.get("Size") {
            if let Ok(size) = size_str.parse::<u64>() {
                info.size = size;
            }
        }
        
        if let Some(installed_size_str) = fields.get("Installed-Size") {
            if let Ok(installed_size) = installed_size_str.parse::<u64>() {
                info.installed_size = installed_size * 1024; // Convert KB to bytes
            }
        }
        
        if let Some(maintainer) = fields.get("Maintainer") {
            info.maintainer = maintainer.clone();
        }
        
        if let Some(homepage) = fields.get("Homepage") {
            info.homepage = Some(homepage.clone());
        }
        
        if let Some(section) = fields.get("Section") {
            info.section = section.clone();
        }
        
        if let Some(priority) = fields.get("Priority") {
            info.priority = priority.clone();
        }
        
        if let Some(filename) = fields.get("Filename") {
            info.filename = filename.clone();
        }
        
        if let Some(md5sum) = fields.get("MD5sum") {
            info.md5sum = md5sum.clone();
        }
        
        if let Some(sha256) = fields.get("SHA256") {
            info.sha256 = sha256.clone();
        }
        
        if let Some(depends) = fields.get("Depends") {
            let deps = depends.split(',')
                .map(|s| s.trim().to_string())
                .collect();
            info.dependencies = deps;
        }
        
        Some(info)
    }

    fn parse_release_file(&self, content: &str) -> Result<HashMap<String, String>> {
        let mut release_info = HashMap::new();
        
        for line in content.lines() {
            if let Some(pos) = line.find(':') {
                let key = line[..pos].trim().to_string();
                let value = line[pos+1..].trim().to_string();
                release_info.insert(key, value);
            }
        }
        
        Ok(release_info)
    }
} 