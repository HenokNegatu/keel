use colored::Colorize;
use crate::models::models::{PackageManager, ProjectConfig};
use std::fs;
use std::process::Command;

pub fn deactiate() {
    let config_str = fs::read_to_string("./project.toml").unwrap();
    let config = match toml::from_str::<ProjectConfig>(config_str.as_str()) {
        Ok(output) => {
            if output.tool.package_manager == PackageManager::Conda
                && output.tool.conda_env_name == None
            {
                panic!("{}", "Oops! `project.toml` [tool] section must have been tampered with!".yellow())
            }
            output
        }
        Err(e) => panic!("{}: {}","error reading".red(), e.to_string().red()),
    };
    match config.tool.package_manager {
        PackageManager::Conda => match conda_deactivate() {
            Ok(_) => println!("{}","deactivated!".green()),
            Err(e) => eprintln!("{}", e),
        },
        PackageManager::Pip => match pip_deactivate() {
            Ok(_) => println!("{}","deactivated!".green()),

            Err(e) => eprintln!("{}", e.to_string().red()),
        },
    }
}

fn conda_deactivate() -> Result<(), String> {
    match Command::new("conda").arg("deactivate").output() {
        Ok(output) if output.status.success() => Ok(()),
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("failed to deactivate: {}", stderr.trim()))
        }
        Err(e) => Err(format!("{}", e)),
    }
}

fn pip_deactivate() -> Result<(), String> {
    match Command::new("deactivate").output() {
        Ok(output) if output.status.success() => Ok(()),
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("failed to dactivate: {}", stderr.trim()))
        }
        Err(e) => Err(format!("{}", e)),
    }
}
