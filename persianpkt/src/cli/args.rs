use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(
    name = "pkt",
    about = "PersianPKT - A modern package manager for Linux distributions",
    version
)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,

    #[clap(long, global = true, help = "Enable verbose output")]
    pub verbose: bool,

    #[clap(long, global = true, help = "Use a specific config file")]
    pub config: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[clap(about = "Install packages")]
    Install {
        #[clap(help = "Packages to install")]
        packages: Vec<String>,

        #[clap(short, long, help = "Don't ask for confirmation")]
        yes: bool,
    },

    #[clap(about = "Remove packages")]
    Remove {
        #[clap(help = "Packages to remove")]
        packages: Vec<String>,

        #[clap(short, long, help = "Don't ask for confirmation")]
        yes: bool,

        #[clap(long, help = "Remove dependencies that are no longer needed")]
        purge: bool,
    },

    #[clap(about = "Update package lists")]
    Update,

    #[clap(about = "Upgrade installed packages")]
    Upgrade {
        #[clap(short, long, help = "Don't ask for confirmation")]
        yes: bool,
    },

    #[clap(about = "Search for packages")]
    Search {
        #[clap(help = "Search query")]
        query: String,
    },

    #[clap(about = "Show package information")]
    Show {
        #[clap(help = "Package name")]
        package: String,
    },

    #[clap(about = "List installed packages")]
    List,

    #[clap(about = "Manage repositories")]
    Repo {
        #[clap(subcommand)]
        command: RepoCommands,
    },

    #[clap(about = "Clean package cache")]
    Clean {
        #[clap(short, long, help = "Remove all cached packages")]
        all: bool,
    },
}

#[derive(Debug, Subcommand)]
pub enum RepoCommands {
    #[clap(about = "Add a repository")]
    Add {
        #[clap(help = "Repository URL")]
        url: String,

        #[clap(help = "Repository name")]
        name: String,
    },

    #[clap(about = "Remove a repository")]
    Remove {
        #[clap(help = "Repository name")]
        name: String,
    },

    #[clap(about = "List all repositories")]
    List,

    #[clap(about = "Enable a repository")]
    Enable {
        #[clap(help = "Repository name")]
        name: String,
    },

    #[clap(about = "Disable a repository")]
    Disable {
        #[clap(help = "Repository name")]
        name: String,
    },
}

pub fn parse_args() -> Args {
    Args::parse()
} 