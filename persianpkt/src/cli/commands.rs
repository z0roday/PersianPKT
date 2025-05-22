use crate::cli::args::{Args, Commands, RepoCommands};
use crate::repository::{Repository, RepositoryManager};
use crate::package::PackageManager;
use crate::config::{Config, ConfigPaths};
use anyhow::{Result, anyhow};
use colored::Colorize;
use std::io::{self, Write};
use tokio::runtime::Runtime;

pub fn execute_command(args: Args) -> Result<()> {
    if args.verbose {
        println!("Verbose mode enabled");
    }

    match args.command {
        Commands::Install { packages, yes } => install_packages(packages, yes),
        Commands::Remove { packages, yes, purge } => remove_packages(packages, yes, purge),
        Commands::Update => update_package_lists(),
        Commands::Upgrade { yes } => upgrade_packages(yes),
        Commands::Search { query } => search_packages(query),
        Commands::Show { package } => show_package_info(package),
        Commands::List => list_installed_packages(),
        Commands::Repo { command } => handle_repo_command(command),
        Commands::Clean { all } => clean_cache(all),
    }
}

fn install_packages(packages: Vec<String>, yes: bool) -> Result<()> {
    if packages.is_empty() {
        return Err(anyhow!("No packages specified for installation"));
    }
    
    println!("{} Installing packages: {}", "==>".green().bold(), packages.join(", "));
    
    if !yes && !confirm_action() {
        println!("Operation cancelled");
        return Ok(());
    }
    
    // Set up configuration
    let config_paths = ConfigPaths::new();
    let config_file = config_paths.config_file();
    let config = Config::load(&config_file)?;
    
    // Load repositories
    let mut repo_manager = RepositoryManager::new(config_paths.repositories_file());
    repo_manager.load_repositories()?;
    let repositories = repo_manager.list_repositories();
    
    if repositories.is_empty() {
        return Err(anyhow!("No repositories configured. Add a repository with 'pkt repo add'"));
    }
    
    // Create package manager
    let package_manager = PackageManager::new(config_paths.packages_dir(), repositories);
    
    // Create async runtime
    let rt = Runtime::new()?;
    
    // Install each package
    for package_name in packages {
        println!("Installing package: {}", package_name);
        
        // Parse version if specified (package_name@version)
        let (name, version) = if package_name.contains('@') {
            let parts: Vec<&str> = package_name.split('@').collect();
            (parts[0].to_string(), Some(parts[1].to_string()))
        } else {
            (package_name, None)
        };
        
        match rt.block_on(package_manager.install_package(&name, version.as_deref())) {
            Ok(_) => println!("{} Successfully installed {}", "✓".green().bold(), name),
            Err(e) => eprintln!("{} Failed to install {}: {}", "✗".red().bold(), name, e),
        }
    }
    
    Ok(())
}

fn remove_packages(packages: Vec<String>, yes: bool, purge: bool) -> Result<()> {
    if packages.is_empty() {
        return Err(anyhow!("No packages specified for removal"));
    }
    
    println!("{} Removing packages: {}", "==>".red().bold(), packages.join(", "));
    
    if purge {
        println!("Unused dependencies will also be removed");
    }
    
    if !yes && !confirm_action() {
        println!("Operation cancelled");
        return Ok(());
    }
    
    println!("Package removal not yet implemented");
    
    Ok(())
}

fn update_package_lists() -> Result<()> {
    println!("{} Updating package lists", "==>".blue().bold());
    
    // Set up configuration
    let config_paths = ConfigPaths::new();
    
    // Load repositories
    let mut repo_manager = RepositoryManager::new(config_paths.repositories_file());
    repo_manager.load_repositories()?;
    let repositories = repo_manager.list_repositories();
    
    if repositories.is_empty() {
        return Err(anyhow!("No repositories configured. Add a repository with 'pkt repo add'"));
    }
    
    // Create async runtime
    let rt = Runtime::new()?;
    
    // Update each repository
    for repo in repositories {
        println!("Updating repository '{}'...", repo.name);
        match rt.block_on(repo.update()) {
            Ok(_) => println!("{} Repository '{}' updated successfully", "✓".green().bold(), repo.name),
            Err(e) => eprintln!("{} Failed to update repository '{}': {}", "✗".red().bold(), repo.name, e),
        }
    }
    
    Ok(())
}

fn upgrade_packages(yes: bool) -> Result<()> {
    println!("{} Upgrading packages", "==>".blue().bold());
    
    if !yes && !confirm_action() {
        println!("Operation cancelled");
        return Ok(());
    }
    
    println!("Package upgrade not yet implemented");
    
    Ok(())
}

fn search_packages(query: String) -> Result<()> {
    if query.is_empty() {
        return Err(anyhow!("Search query cannot be empty"));
    }
    
    println!("{} Searching for packages matching: {}", "==>".blue().bold(), query);
    
    // Set up configuration
    let config_paths = ConfigPaths::new();
    
    // Load repositories
    let mut repo_manager = RepositoryManager::new(config_paths.repositories_file());
    repo_manager.load_repositories()?;
    let repositories = repo_manager.list_repositories();
    
    if repositories.is_empty() {
        return Err(anyhow!("No repositories configured. Add a repository with 'pkt repo add'"));
    }
    
    // Create async runtime
    let rt = Runtime::new()?;
    
    // Search in each repository
    let mut found = false;
    
    for repo in repositories {
        match rt.block_on(repo.search_packages(&query)) {
            Ok(packages) => {
                if !packages.is_empty() {
                    found = true;
                    println!("\nPackages found in repository '{}':", repo.name);
                    
                    for package in packages {
                        println!("  {} (v{}) - {}", 
                            package.info.name.bold(), 
                            package.info.version,
                            package.info.description);
                    }
                }
            },
            Err(e) => eprintln!("Error searching in repository {}: {}", repo.name, e),
        }
    }
    
    if !found {
        println!("No packages found matching '{}'", query);
    }
    
    Ok(())
}

fn show_package_info(package: String) -> Result<()> {
    if package.is_empty() {
        return Err(anyhow!("Package name cannot be empty"));
    }
    
    println!("{} Package information for: {}", "==>".blue().bold(), package);
    
    // Set up configuration
    let config_paths = ConfigPaths::new();
    
    // Load repositories
    let mut repo_manager = RepositoryManager::new(config_paths.repositories_file());
    repo_manager.load_repositories()?;
    let repositories = repo_manager.list_repositories();
    
    if repositories.is_empty() {
        return Err(anyhow!("No repositories configured. Add a repository with 'pkt repo add'"));
    }
    
    // Create package manager
    let package_manager = PackageManager::new(config_paths.packages_dir(), repositories);
    
    // Create async runtime
    let rt = Runtime::new()?;
    
    // Find package
    match rt.block_on(package_manager.find_package(&package, None)) {
        Ok(pkg) => {
            println!("Name: {}", pkg.name.bold());
            println!("Version: {}", pkg.version);
            println!("Architecture: {}", pkg.architecture);
            println!("Description: {}", pkg.description);
            println!("Maintainer: {}", pkg.maintainer);
            
            if let Some(homepage) = &pkg.homepage {
                println!("Homepage: {}", homepage);
            }
            
            println!("Section: {}", pkg.section);
            println!("Priority: {}", pkg.priority);
            println!("Size: {} bytes", pkg.size);
            println!("Installed Size: {} bytes", pkg.installed_size);
            
            if !pkg.dependencies.is_empty() {
                println!("\nDependencies:");
                for dep in &pkg.dependencies {
                    println!("  {}", dep.name);
                }
            }
            
            if !pkg.conflicts.is_empty() {
                println!("\nConflicts:");
                for conflict in &pkg.conflicts {
                    println!("  {}", conflict);
                }
            }
        },
        Err(e) => {
            return Err(anyhow!("Package '{}' not found: {}", package, e));
        }
    }
    
    Ok(())
}

fn list_installed_packages() -> Result<()> {
    println!("{} Installed packages:", "==>".blue().bold());
    
    // Set up configuration
    let config_paths = ConfigPaths::new();
    
    // Check if packages directory exists
    let packages_dir = config_paths.packages_dir();
    if !packages_dir.exists() {
        println!("No packages installed");
        return Ok(());
    }
    
    // List installed packages
    let entries = std::fs::read_dir(packages_dir)?;
    let mut found = false;
    
    for entry in entries {
        if let Ok(entry) = entry {
            if entry.file_type()?.is_dir() {
                found = true;
                let package_name = entry.file_name();
                let package_dir = entry.path();
                
                // List versions
                if let Ok(versions) = std::fs::read_dir(package_dir) {
                    for version in versions {
                        if let Ok(version) = version {
                            if version.file_type()?.is_dir() {
                                println!("  {} (v{})", 
                                    package_name.to_string_lossy().bold(),
                                    version.file_name().to_string_lossy());
                            }
                        }
                    }
                }
            }
        }
    }
    
    if !found {
        println!("No packages installed");
    }
    
    Ok(())
}

fn handle_repo_command(command: RepoCommands) -> Result<()> {
    match command {
        RepoCommands::Add { url, name } => add_repository(url, name),
        RepoCommands::Remove { name } => remove_repository(name),
        RepoCommands::List => list_repositories(),
        RepoCommands::Enable { name } => enable_repository(name),
        RepoCommands::Disable { name } => disable_repository(name),
    }
}

fn add_repository(url: String, name: String) -> Result<()> {
    if name.is_empty() {
        return Err(anyhow!("Repository name cannot be empty"));
    }
    
    if url.is_empty() {
        return Err(anyhow!("Repository URL cannot be empty"));
    }
    
    println!("{} Adding repository: {} ({})", "==>".blue().bold(), name, url);
    
    let config_paths = ConfigPaths::new();
    let mut repo_manager = RepositoryManager::new(config_paths.repositories_file());
    
    // Load existing repositories
    repo_manager.load_repositories()?;
    
    // Create new repository
    let repo = Repository::new(name.clone(), &url)?;
    
    // Add repository
    match repo_manager.add_repository(repo) {
        Ok(_) => {
            println!("{} Repository {} successfully added", "✓".green().bold(), name);
            Ok(())
        },
        Err(e) => Err(anyhow!("Error adding repository {}: {}", name, e))
    }
}

fn remove_repository(name: String) -> Result<()> {
    if name.is_empty() {
        return Err(anyhow!("Repository name cannot be empty"));
    }
    
    println!("{} Removing repository: {}", "==>".red().bold(), name);
    
    let config_paths = ConfigPaths::new();
    let mut repo_manager = RepositoryManager::new(config_paths.repositories_file());
    
    // Load existing repositories
    repo_manager.load_repositories()?;
    
    // Remove repository
    match repo_manager.remove_repository(&name) {
        Ok(true) => {
            println!("{} Repository {} successfully removed", "✓".green().bold(), name);
            Ok(())
        },
        Ok(false) => {
            println!("{} Repository {} not found", "✗".red().bold(), name);
            Ok(())
        },
        Err(e) => Err(anyhow!("Error removing repository {}: {}", name, e))
    }
}

fn list_repositories() -> Result<()> {
    println!("{} Available repositories:", "==>".blue().bold());
    
    let config_paths = ConfigPaths::new();
    let mut repo_manager = RepositoryManager::new(config_paths.repositories_file());
    
    // Load existing repositories
    repo_manager.load_repositories()?;
    
    let repos = repo_manager.list_repositories();
    
    if repos.is_empty() {
        println!("No repositories found");
    } else {
        for repo in repos {
            let status = if repo.enabled {
                "[Enabled]".green()
            } else {
                "[Disabled]".red()
            };
            
            println!("{} {} ({}): {}", status, repo.name, repo.priority, repo.url);
        }
    }
    
    Ok(())
}

fn enable_repository(name: String) -> Result<()> {
    if name.is_empty() {
        return Err(anyhow!("Repository name cannot be empty"));
    }
    
    println!("{} Enabling repository: {}", "==>".green().bold(), name);
    
    let config_paths = ConfigPaths::new();
    let mut repo_manager = RepositoryManager::new(config_paths.repositories_file());
    
    // Load existing repositories
    repo_manager.load_repositories()?;
    
    // Enable repository
    match repo_manager.enable_repository(&name) {
        Ok(true) => {
            println!("{} Repository {} successfully enabled", "✓".green().bold(), name);
            Ok(())
        },
        Ok(false) => {
            println!("{} Repository {} not found", "✗".red().bold(), name);
            Ok(())
        },
        Err(e) => Err(anyhow!("Error enabling repository {}: {}", name, e))
    }
}

fn disable_repository(name: String) -> Result<()> {
    if name.is_empty() {
        return Err(anyhow!("Repository name cannot be empty"));
    }
    
    println!("{} Disabling repository: {}", "==>".yellow().bold(), name);
    
    let config_paths = ConfigPaths::new();
    let mut repo_manager = RepositoryManager::new(config_paths.repositories_file());
    
    // Load existing repositories
    repo_manager.load_repositories()?;
    
    // Disable repository
    match repo_manager.disable_repository(&name) {
        Ok(true) => {
            println!("{} Repository {} successfully disabled", "✓".green().bold(), name);
            Ok(())
        },
        Ok(false) => {
            println!("{} Repository {} not found", "✗".red().bold(), name);
            Ok(())
        },
        Err(e) => Err(anyhow!("Error disabling repository {}: {}", name, e))
    }
}

fn clean_cache(all: bool) -> Result<()> {
    println!("{} Cleaning package cache", "==>".blue().bold());
    
    // Set up configuration
    let config_paths = ConfigPaths::new();
    let temp_dir = config_paths.packages_dir().join("temp");
    
    if all {
        println!("Removing all cached packages...");
        if temp_dir.exists() {
            std::fs::remove_dir_all(&temp_dir)?;
            std::fs::create_dir_all(&temp_dir)?;
            println!("{} All cached packages removed", "✓".green().bold());
        } else {
            println!("Cache directory does not exist");
        }
    } else {
        println!("Removing temporary packages...");
        if temp_dir.exists() {
            let entries = std::fs::read_dir(&temp_dir)?;
            let mut count = 0;
            
            for entry in entries {
                if let Ok(entry) = entry {
                    std::fs::remove_file(entry.path())?;
                    count += 1;
                }
            }
            
            println!("{} Removed {} cached packages", "✓".green().bold(), count);
        } else {
            println!("Cache directory does not exist");
        }
    }
    
    Ok(())
}

fn confirm_action() -> bool {
    print!("Do you want to continue? [Y/n] ");
    std::io::stdout().flush().unwrap();
    
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return false;
    }
    
    let input = input.trim().to_lowercase();
    input.is_empty() || input == "y" || input == "yes"
} 