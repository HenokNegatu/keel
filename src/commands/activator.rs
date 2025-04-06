use crate::models::models::{PackageManager, ProjectConfig};
use std::{fs, process::Command, process::Stdio};

pub fn activate() {
    let config_str = fs::read_to_string("./project.toml").unwrap();
    let config = match toml::from_str::<ProjectConfig>(config_str.as_str()) {
        Ok(output) => {
            if output.tool.package_manager == PackageManager::Conda && output.tool.conda_env_name == None {
                panic!("Oops! `project.toml` [tool] section must have been tampered with!")
            }
            output},
        Err(e) => panic!("error reading {}", e),
    };
    match config.tool.package_manager {
        PackageManager::Conda => {
            match conda_activate(&config.tool.conda_env_name.unwrap()) {
                Ok(_) => println!("activated!"),
                Err(e) => eprintln!("{}", e)
            }
        }
        PackageManager::Pip => match pip_activate() {
            Ok(_) => println!("activated!"),
            
            Err(e) => eprintln!("{}", e)
        }
    }
}

#[cfg(windows)]
fn pip_activate() -> Result<(), String>{
    match Command::new("./Scripts/activate").output() {
        Ok(output) if output.status.success() => Ok(()),
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("failed to activate: {}", stderr.trim());
            Err(format!("failed to activate: {}", stderr.trim()))
        }
        Err(e) => Err(format!("{}",e))
    }
}


fn conda_activate(env_name: &str) -> std::io::Result<()>{
    let conda_path = "~/miniconda3/bin/conda";
    let shell_cmd = if cfg!(windows) {
        format!(
            r#"
            @echo off
            call "{}" activate {}
            pause
            "#,
            conda_path.replace('~', "%USERPROFILE%"),
            env_name
        )
    } else {
        format!(
            r#"
            source ~/miniconda3/etc/profile.d/conda.sh
            conda activate {}
            bash
            "#,
            env_name,
        )
    };

    let shell = if cfg!(windows) { "cmd.exe" } else { "bash" };
    let flag = if cfg!(windows) { "/C" } else { "-c" };

    Command::new(shell)
        .arg(flag)
        .arg(&shell_cmd)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?
        .wait()?;

    Ok(())
}

#[cfg(unix)]
fn pip_activate() -> Result<(), String>{
    match Command::new("./bin/activate").output() {
        Ok(output) if output.status.success() => Ok(()),
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("failed to activate: {}", stderr.trim());
            Err(format!("failed to activate: {}", stderr.trim()))
        }
        Err(e) => Err(format!("{}",e))
    }
}