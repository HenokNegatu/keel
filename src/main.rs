mod commands;
mod models;
mod utils;

use crate::models::models::{Cli, Commands};
use clap::Parser;
use commands::{activator::activate, deactivator::deactiate, new::new};

use std::process;
fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::New => new(),
        Commands::Activate => activate(),
        Commands::Deactivate => deactiate(),
    }
    process::exit(0);
    
}
