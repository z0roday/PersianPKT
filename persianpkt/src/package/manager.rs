use anyhow::{Result, anyhow};
use std::path::PathBuf;
use std::fs;
use reqwest::Client;
use futures_util::StreamExt;
use tokio::io::AsyncWriteExt;
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::Arc;
use std::time::SystemTime;

use crate::package::{Package, PackageInfo};
use crate::repository::Repository;

pub struct PackageManager {
    client: Client,
    install_dir: PathBuf,
    repositories: Vec<Repository>,
}

impl PackageManager {
    pub fn new(install_dir: PathBuf, repositories: Vec<Repository>) -> Self {
        Self {
            client: Client::new(),
            install_dir,
            repositories,
        }
    }

    pub async fn download_package(&self, package_name: &str, version: Option<&str>) -> Result<(PathBuf, Package)> {
        let package = self.find_package(package_name, version).await?;
        
        // Create temp directory if it doesn't exist
        let temp_dir = self.install_dir.join("temp");
        if !temp_dir.exists() {
            fs::create_dir_all(&temp_dir)?;
        }
        
        let download_path = temp_dir.join(format!("{}_{}.pkg", package.name, package.version));
        
        // Download the package
        println!("Downloading {} version {}...", package.name, package.version);
        
        // This is a stub for demonstration
        // In a real implementation, we would get the URL from the package metadata
        let url = format!("https://example.com/{}/{}", package.name, package.version);
        
        let res = self.client.get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to download package: {}", e))?;
            
        let total_size = res.content_length().unwrap_or(0);
        
        let pb = ProgressBar::new(total_size);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("#>-"));
            
        let mut file = tokio::fs::File::create(&download_path).await?;
        let mut downloaded: u64 = 0;
        let mut stream = res.bytes_stream();
        
        while let Some(item) = stream.next().await {
            let chunk = item.map_err(|e| anyhow!("Error while downloading: {}", e))?;
            file.write_all(&chunk).await?;
            
            downloaded += chunk.len() as u64;
            pb.set_position(downloaded);
        }
        
        pb.finish_with_message("Download completed");
        
        Ok((download_path, package))
    }
    
    pub async fn install_package(&self, package_name: &str, version: Option<&str>) -> Result<()> {
        let (package_path, package) = self.download_package(package_name, version).await?;
        
        println!("Installing {} package...", package_name);
        
        // Create package directory
        let install_path = self.install_dir.join(&package.name).join(&package.version);
        
        if !install_path.exists() {
            fs::create_dir_all(&install_path)?;
        }
        
        // Extract the package - simulated for demonstration
        // In a real implementation, we would extract the package file
        let installed_package = Package {
            name: package.name.clone(),
            version: package.version.clone(),
            architecture: package.architecture.clone(),
            description: package.description.clone(),
            dependencies: package.dependencies.clone(),
            conflicts: package.conflicts.clone(),
            provides: package.provides.clone(),
            replaces: package.replaces.clone(),
            install_path: install_path.clone(),
            files: Vec::new(), // Would be populated from the package contents
            install_date: SystemTime::now(),
            size: package.size,
            installed_size: package.installed_size,
            maintainer: package.maintainer.clone(),
            homepage: package.homepage.clone(),
            section: package.section.clone(),
            priority: package.priority.clone(),
        };
        
        // Save package metadata
        let metadata_path = install_path.join("package.json");
        installed_package.save_to_file(&metadata_path)?;
        
        println!("Package {} v{} has been successfully installed", package.name, package.version);
        
        // Clean up temporary files
        fs::remove_file(package_path)?;
        
        Ok(())
    }
    
    pub async fn find_package(&self, package_name: &str, version: Option<&str>) -> Result<Package> {
        for repo in &self.repositories {
            if let Ok(packages) = repo.search_packages(package_name).await {
                if !packages.is_empty() {
                    // If version is specified, find that specific version
                    if let Some(version_req) = version {
                        for package in packages {
                            if package.name == package_name && package.version == version_req {
                                return Ok(package);
                            }
                        }
                    } else {
                        // Find the latest version
                        let mut latest_package = packages[0].clone();
                        for package in packages {
                            if package.name == package_name && 
                               semver::Version::parse(&package.version).unwrap() > 
                               semver::Version::parse(&latest_package.version).unwrap() {
                                latest_package = package;
                            }
                        }
                        return Ok(latest_package);
                    }
                }
            }
        }
        
        Err(anyhow!("Package {} not found", package_name))
    }
} 