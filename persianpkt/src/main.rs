mod cli;
mod core;
mod repository;
mod package;
mod utils;
mod config;

use anyhow::{Result, anyhow};
use config::{Config, ConfigPaths};
use colored::Colorize;

fn main() -> Result<()> {
    env_logger::init();
    
    let config_paths = ConfigPaths::new();
    config_paths.ensure_dirs_exist()?;
    
    let config_file = config_paths.config_file();
    let config = Config::load(&config_file)?;
    
    match cli::run() {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), e);
            Err(anyhow!("PersianPKT exited with an error"))
        }
    }
}
