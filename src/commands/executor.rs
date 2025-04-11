use colored::Colorize;
use crate::models::models::PackageManager;
use std::process::Command;
use crate::models::error::VenvError;


pub fn create_venv(
    project_name: &str,
    package_manager: &PackageManager,
    env_name: &Option<String>,
) -> Result<(), VenvError> {
    match package_manager {
        PackageManager::Pip => {
            check_venv()?;
            if cfg!(target_os = "windows") {
                Command::new("python")
                    .args(&["-m", "venv", project_name])
                    .output()
                    .map_err(|e| VenvError::VenvCreationFailed { source: e })?;
                println!("{}: {}", "Virtual environment created at".dimmed(), project_name);
                    Ok(())
            } else {
                Command::new("python3")
                    .args(&["-m", "venv", project_name])
                    .output()
                    .map_err(|e| VenvError::VenvCreationFailed { source: e })?;
                println!("{}: {}", "Virtual environment created at".dimmed(), project_name);
                Ok(())
            }
        },
        PackageManager::Conda => {
            check_conda()?;
            println!("running conda...");
            let conda_env = match &env_name {
                Some(name) => name.as_str(),
                None => project_name,
            };
            Command::new("conda")
                .args(&["create", "-n", conda_env, "-y"])
                .output().map_err(|e| VenvError::CondaCreationFailed { source: e })?;
            println!("{}: {}", "Conda environment created at".dimmed(), conda_env);
            Ok(())
        }
    }
}

fn check_venv() -> Result<(), VenvError> {
    let python_cmd = if cfg!(target_os = "windows") {
        "python"
    } else {
        "python3"
    };
    println!("{}", "Checking for venv...".dimmed());
    Command::new(python_cmd)
        .args(&["-m", "venv", "--help"])
        .output()
        .map_err(|e| {
            eprintln!(
                "{}",
                "Python venv module is not installed or not working properly.".red()
            );
            if cfg!(target_os = "linux") {
                eprintln!("   {}", "sudo apt install python3-venv".yellow());
            } else if cfg!(target_os = "macos") {
                eprintln!("   {}", "brew install python (or reinstall Python)".yellow());
            } else if cfg!(target_os = "windows") {
                eprintln!("   {}", "Ensure Python is installed correctly and added to PATH.".yellow());
            } else {
                eprintln!("   {}", "Ensure Python is installed correctly.".yellow());
            }
            VenvError::VenvCheckFailed { source: e }
        })?;
    Ok(())
}

fn check_conda() -> Result<(), VenvError> {
    Command::new("conda")
        .args(&["--version"])
        .output()
        .map_err(|e| {
            eprintln!(
                "{}",
                "Conda is not installed! Install it from: https://docs.conda.io/en/latest/miniconda.html"
                    .red()
            );
            VenvError::CondaCheckFailed { source: e }
        })?;
    Ok(())
}
