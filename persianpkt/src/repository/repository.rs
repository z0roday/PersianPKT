use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use url::Url;
use std::path::PathBuf;
use std::fs;
use reqwest::Client;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub name: String,
    pub url: Url,
    pub enabled: bool,
    pub priority: i32,
    pub distribution: String,
    pub components: Vec<String>,
    pub architectures: Vec<String>,
}

impl Repository {
    pub fn new(name: String, url_str: &str) -> Result<Self> {
        let url = Url::parse(url_str).map_err(|e| anyhow!("Invalid URL: {}", e))?;
        
        // Validate URL scheme is http or https
        if url.scheme() != "http" && url.scheme() != "https" {
            return Err(anyhow!("Invalid URL scheme. Only http and https are supported"));
        }
        
        // Validate the URL has a host
        if url.host().is_none() {
            return Err(anyhow!("URL must have a host"));
        }
        
        Ok(Self {
            name,
            url,
            enabled: true,
            priority: 100,
            distribution: "stable".to_string(),
            components: vec!["main".to_string()],
            architectures: vec!["amd64".to_string(), "i386".to_string()],
        })
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn set_priority(&mut self, priority: i32) {
        self.priority = priority;
    }

    pub fn add_component(&mut self, component: String) {
        if !self.components.contains(&component) {
            self.components.push(component);
        }
    }

    pub fn remove_component(&mut self, component: &str) {
        self.components.retain(|c| c != component);
    }

    pub fn add_architecture(&mut self, arch: String) {
        if !self.architectures.contains(&arch) {
            self.architectures.push(arch);
        }
    }

    pub fn remove_architecture(&mut self, arch: &str) {
        self.architectures.retain(|a| a != arch);
    }

    pub fn get_package_list_url(&self, component: &str, arch: &str) -> Result<Url> {
        self.url.join(&format!(
            "dists/{}/{}/{}/Packages",
            self.distribution, component, arch
        )).map_err(|e| anyhow!("Failed to create package list URL: {}", e))
    }

    pub fn get_release_url(&self) -> Result<Url> {
        self.url.join(&format!("dists/{}/Release", self.distribution))
            .map_err(|e| anyhow!("Failed to create release URL: {}", e))
    }

    pub async fn update(&self) -> Result<()> {
        let client = Client::new();
        
        println!("Fetching repository information from {}", self.url);
        
        // Get release file
        let release_url = self.get_release_url()?;
        let response = client.get(release_url)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to fetch release file: {}", e))?;
            
        if !response.status().is_success() {
            return Err(anyhow!("Failed to fetch release file: HTTP {}", response.status()));
        }
        
        println!("Successfully connected to repository");
        
        // Update package lists for each component and architecture
        for component in &self.components {
            for arch in &self.architectures {
                let packages_url = self.get_package_list_url(component, arch)?;
                
                println!("Fetching package list for {}/{}", component, arch);
                
                let response = client.get(packages_url)
                    .send()
                    .await
                    .map_err(|e| anyhow!("Failed to fetch package list: {}", e))?;
                    
                if !response.status().is_success() {
                    println!("Warning: Could not fetch package list for {}/{}: HTTP {}", 
                             component, arch, response.status());
                    continue;
                }
                
                println!("Package list for {}/{} updated", component, arch);
            }
        }
        
        println!("Repository {} update completed", self.name);
        
        Ok(())
    }
    
    pub async fn search_packages(&self, query: &str) -> Result<Vec<crate::package::Package>> {
        // This is a stub implementation
        // In a real implementation, we would search the downloaded package lists
        
        if !self.enabled {
            return Ok(Vec::new());
        }
        
        if query.is_empty() {
            return Ok(Vec::new());
        }
        
        // Create a dummy package for demonstration
        let mut packages = Vec::new();
        
        if "test".contains(query) || query.contains("test") {
            let package = crate::package::Package {
                name: "test-package".to_string(),
                version: "1.0.0".to_string(),
                architecture: "x86_64".to_string(),
                description: "A test package for development".to_string(),
                dependencies: Vec::new(),
                conflicts: Vec::new(),
                provides: Vec::new(),
                replaces: Vec::new(),
                install_path: std::path::PathBuf::new(),
                files: Vec::new(),
                install_date: std::time::SystemTime::now(),
                size: 1024,
                installed_size: 2048,
                maintainer: "PersianPKT Team".to_string(),
                homepage: Some("https://example.com/test-package".to_string()),
                section: "development".to_string(),
                priority: "optional".to_string(),
            };
            
            packages.push(package);
        }
        
        Ok(packages)
    }
}

pub struct RepositoryManager {
    repositories: Vec<Repository>,
    config_path: PathBuf,
}

impl RepositoryManager {
    pub fn new(config_path: PathBuf) -> Self {
        Self {
            repositories: Vec::new(),
            config_path,
        }
    }

    pub fn load_repositories(&mut self) -> Result<()> {
        if !self.config_path.exists() {
            return Ok(());
        }

        let content = fs::read_to_string(&self.config_path)
            .map_err(|e| anyhow!("Failed to read repository configuration file: {}", e))?;
            
        if content.trim().is_empty() {
            return Ok(());
        }
        
        let repos: Vec<Repository> = serde_json::from_str(&content)
            .map_err(|e| anyhow!("Failed to parse repository configuration file: {}", e))?;
            
        self.repositories = repos;
        Ok(())
    }

    pub fn save_repositories(&self) -> Result<()> {
        // Create parent directories if they don't exist
        if let Some(parent) = self.config_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .map_err(|e| anyhow!("Failed to create parent directories: {}", e))?;
            }
        }
        
        let content = serde_json::to_string_pretty(&self.repositories)
            .map_err(|e| anyhow!("Failed to convert repositories to JSON: {}", e))?;
            
        fs::write(&self.config_path, content)
            .map_err(|e| anyhow!("Failed to save repository configuration file: {}", e))?;
            
        Ok(())
    }

    pub fn add_repository(&mut self, repo: Repository) -> Result<()> {
        // Check if repository with same name already exists
        if let Some(existing) = self.get_repository(&repo.name) {
            return Err(anyhow!("Repository with name {} already exists", existing.name));
        }
        
        self.repositories.push(repo);
        self.save_repositories()?;
        Ok(())
    }

    pub fn remove_repository(&mut self, name: &str) -> Result<bool> {
        let initial_len = self.repositories.len();
        self.repositories.retain(|r| r.name != name);
        let removed = self.repositories.len() < initial_len;
        
        if removed {
            self.save_repositories()?;
        }
        
        Ok(removed)
    }

    pub fn get_repository(&self, name: &str) -> Option<&Repository> {
        self.repositories.iter().find(|r| r.name == name)
    }

    pub fn get_repository_mut(&mut self, name: &str) -> Option<&mut Repository> {
        self.repositories.iter_mut().find(|r| r.name == name)
    }

    pub fn list_repositories(&self) -> &[Repository] {
        &self.repositories
    }

    pub fn list_enabled_repositories(&self) -> Vec<&Repository> {
        self.repositories.iter().filter(|r| r.enabled).collect()
    }
    
    pub fn enable_repository(&mut self, name: &str) -> Result<bool> {
        if let Some(repo) = self.get_repository_mut(name) {
            repo.enable();
            self.save_repositories()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    pub fn disable_repository(&mut self, name: &str) -> Result<bool> {
        if let Some(repo) = self.get_repository_mut(name) {
            repo.disable();
            self.save_repositories()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
} 