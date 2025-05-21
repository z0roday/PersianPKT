mod commands;
mod args;

pub use args::*;
pub use commands::*;

pub fn run() -> anyhow::Result<()> {
    let args = args::parse_args();
    commands::execute_command(args)
} 