mod commands;
mod models;
mod utils;

use crate::models::models::{Cli, Commands};
use clap::Parser;
use commands::{new::new, activator::activate};

use std::process;
fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::New => new(),
        Commands::Activate => activate(),
        Commands::Deactivate => println!("deactiate")
    }
    process::exit(0);
    
}
