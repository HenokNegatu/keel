use crate::models::models::PackageManager;
use std::process::Command;

pub fn create_venv(project_name: &str, package_manager: &PackageManager, env_name: &Option<String>) {
    match package_manager {
        PackageManager::Pip => match check_venv() {
            Ok(_) => {
                if cfg!(target_os = "windows") {
                    match Command::new("python").args(&["-m", "venv", project_name]).output() {
                        Ok(output) if output.status.success() => println!("venv created!"),
                        Ok(output) => {
                            let stderr = String::from_utf8_lossy(&output.stderr);
                            eprintln!("failed to create env: {}", stderr.trim());
                        }
                        Err(e) => eprintln!("{}", e)
                    }
                } else {
                    match Command::new("python3").args(&["-m", "venv", project_name]).output() {
                        Ok(output) if output.status.success() => println!("venv created!"),
                        Ok(output) => {
                            let stderr = String::from_utf8_lossy(&output.stderr);
                            eprintln!("failed to create env: {}", stderr.trim());
                        },
                        Err(e) => eprintln!("{}", e)
                    }
                };
            }
            Err(e) => eprintln!("{}",e),
        },
        PackageManager::Conda => match check_conda() {
            Ok(_) => {
                println!("running conda...");
                let conda_env = match &env_name {
                    Some(name) => name.as_str(),
                    None => project_name,
                };
                match Command::new("conda").args(&["create", "-n", conda_env, "-y"]).output() {
                    Ok(output) if output.status.success()=> println!("conda env created"),
                    Ok(output) => {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        eprintln!("failed to create env {}", stderr);
                    },
                    Err(e) => println!("{}", e)
                }
            }
            Err(e) => eprintln!("{}", e),
        },
    }
}

fn check_venv() -> Result<(), String> {
    let python_cmd = if cfg!(target_os = "windows") {
        "python"
    } else {
        "python3"
    };
    println!("checking venv...");
    let check_venv = Command::new(python_cmd)
        .args(&["-m", "venv", "--help"])
        .output();

    match check_venv {
        Ok(output) if output.status.success() => {
            println!(" `venv` is installed! Creating virtual environment...");
            Ok(())
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("venv check failed: {}", stderr.trim());
            Err(format!("venv check failed: {}", stderr.trim()))
        }
        Err(e) => {
            if cfg!(target_os = "linux") {
                eprintln!("   sudo apt install python3-venv");
            } else if cfg!(target_os = "macos") {
                eprintln!("   brew install python (or reinstall Python)");
            } else {
                eprintln!("   Ensure Python is installed correctly.");
            };
            Err(format!(
                "`venv` is not installed! Try installing it: {}",
                e
            ))
        }
    }
}

fn check_conda() -> Result<(), String> {
    let check_conda = Command::new("conda").args(&["--version"]).output();
    match check_conda {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            println!("Conda is installed! Version: {}", version.trim());
            Ok(())
        },
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("Conda check failed: {}", stderr.trim());
            Err(format!("Conda check failed: {}", stderr.trim()))
        },
        Err(e) => {
            Err(format!("Conda is not installed! Install it from: https://docs.conda.io/en/latest/miniconda.html, {}", e))   
        }
    }
}

