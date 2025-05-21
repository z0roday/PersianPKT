use anyhow::Result;
use std::path::PathBuf;
use crate::repository::Repository;
use crate::package::{Package, PackageInfo};

pub struct PackageManager {
    repositories: Vec<Repository>,
    cache_dir: PathBuf,
    config_dir: PathBuf,
}

impl PackageManager {
    pub fn new(config_dir: PathBuf, cache_dir: PathBuf) -> Self {
        Self {
            repositories: Vec::new(),
            cache_dir,
            config_dir,
        }
    }

    pub fn load_repositories(&mut self) -> Result<()> {

        Ok(())
    }

    pub fn install_packages(&self, packages: &[String], yes: bool) -> Result<()> {
        Ok(())
    }

    pub fn remove_packages(&self, packages: &[String], purge: bool) -> Result<()> {
        Ok(())
    }

    pub fn update_package_lists(&self) -> Result<()> {

        Ok(())
    }

    pub fn upgrade_packages(&self) -> Result<()> {

        Ok(())
    }

    pub fn search_packages(&self, query: &str) -> Result<Vec<PackageInfo>> {

        Ok(Vec::new())
    }

    pub fn get_package_info(&self, package_name: &str) -> Result<Option<PackageInfo>> {

        Ok(None)
    }

    pub fn list_installed_packages(&self) -> Result<Vec<Package>> {

        Ok(Vec::new())
    }

    pub fn add_repository(&mut self, url: &str, name: &str) -> Result<()> {

        Ok(())
    }

    pub fn remove_repository(&mut self, name: &str) -> Result<()> {

        Ok(())
    }

    pub fn list_repositories(&self) -> Vec<&Repository> {

        Vec::new()
    }

    pub fn enable_repository(&mut self, name: &str) -> Result<()> {

        Ok(())
    }

    pub fn disable_repository(&mut self, name: &str) -> Result<()> {

        Ok(())
    }

    pub fn clean_cache(&self, all: bool) -> Result<()> {
        Ok(())
    }
} 