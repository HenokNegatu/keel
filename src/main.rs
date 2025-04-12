mod commands;
mod models;
mod utils;

use crate::models::models::{Cli, Commands};
use clap::Parser;
use commands::{activator::activate, deactivator::deactiate, new::new};
use colored::Colorize;

use std::process;

fn print_header() {
    println!(
        r#"
{b}      ___           ___           ___           ___ 
     /\__\         /\  \         /\  \         /\__\
    /:/  /        /::\  \       /::\  \       /:/  /
   /:/__/        /:/\:\  \     /:/\:\  \     /:/  / 
  /::\__\____   /::\~\:\  \   /::\~\:\  \   /:/  /  
 /:/\:::::\__\ /:/\:\ \:\__\ /:/\:\ \:\__\ /:/__/   
 \/_|:|~~|~    \:\~\:\ \/__/ \:\~\:\ \/__/ \:\  \   
    |:|  |      \:\ \:\__\    \:\ \:\__\    \:\  \  
    |:|  |       \:\ \/__/     \:\ \/__/     \:\  \ 
    |:|  |        \:\__\        \:\__\        \:\__\
     \|__|         \/__/         \/__/         \/__/{y}⚡{w} {c}v0.1.0-alpha.1{w}

{lg}➤ Python Project Generator on top of {lb}pip{w} 🐍 & {lb}conda{w} 📦
{y}➤ {w}GitHub: {lb}https://github.com/HenokNegatu/keel{w}
{y}➤ {w}License: {lb}MIT | {m}Ctrl+C{w} to exit | {m}--help{w} Options
"#,
        // ANSI Color Codes
        b = "\x1b[34m",     // Blue
        lb = "\x1b[94m",    // Light Blue
        y = "\x1b[33m",     // Yellow
        m = "\x1b[35m",     // Magenta
        c = "\x1b[36m",     // Cyan
        lg = "\x1b[92m",    // Light Green
        w = "\x1b[0m"       // Reset
            );
}


fn main() {
    print_header();
    let cli = Cli::parse();
    match &cli.command {
        Commands::New => match new() {
            Ok(_) => println!("{}", "Project created successfully!".green()),
            Err(e) => eprintln!("{} {:#}", "Aborting.".red(), format!("{:?}", e))
        },
        Commands::Activate => activate(),
        Commands::Deactivate => deactiate(),
    }
    process::exit(0);
    
}
