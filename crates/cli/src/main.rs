use clap::Parser;
use commands::{Cli, Commands};

pub mod accounts;
pub mod commands;
pub mod events;
pub mod handlers;
pub mod idl;
pub mod instructions;
pub mod legacy_idl;
pub mod types;
pub mod util;

type Result<T> = anyhow::Result<T>;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Parse(options) => handlers::parse(options)?,
    };

    Ok(())
}
