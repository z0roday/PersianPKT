use anyhow::Result;
use serde::{Serialize, Deserialize};
use url::Url;
use std::path::PathBuf;
use std::fs;

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
    pub fn new(name: String, url: Url) -> Self {
        Self {
            name,
            url,
            enabled: true,
            priority: 100,
            distribution: "stable".to_string(),
            components: vec!["main".to_string()],
            architectures: vec!["amd64".to_string(), "i386".to_string()],
        }
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
        let url = self.url.join(&format!(
            "dists/{}/{}/{}/Packages",
            self.distribution, component, arch
        ))?;
        Ok(url)
    }

    pub fn get_release_url(&self) -> Result<Url> {
        let url = self.url.join(&format!("dists/{}/Release", self.distribution))?;
        Ok(url)
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

        let content = fs::read_to_string(&self.config_path)?;
        let repos: Vec<Repository> = serde_json::from_str(&content)?;
        self.repositories = repos;
        Ok(())
    }

    pub fn save_repositories(&self) -> Result<()> {
        let content = serde_json::to_string_pretty(&self.repositories)?;
        fs::write(&self.config_path, content)?;
        Ok(())
    }

    pub fn add_repository(&mut self, repo: Repository) -> Result<()> {
        if self.repositories.iter().any(|r| r.name == repo.name) {
            return Ok(());
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
} 