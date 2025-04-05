use crate::models::models::ProjectConfig;
use crate::utils::sample_text;
use std::fs::{self, File};
use std::io::Write;
use std::{io, process::Command};


pub fn create_folder_and_file(folder_path: &str) -> std::io::Result<()> {
    let src_folder_path = format!("{}/src", folder_path);
    let asset_path = format!("./{}/assets", folder_path);
    let file_path = format!("./{}/src/main.py", folder_path);
    let todo_file_path = format!("./{}/TODO.md", folder_path);

    fs::create_dir_all(&src_folder_path)?;
    fs::create_dir_all(&asset_path)?;

    let mut file = File::create(&file_path)?;

    writeln!(file, "{}", sample_text::PYTHON_CODE)?;

    let mut todo_file = File::create(&todo_file_path)?;
    writeln!(todo_file, "{}", sample_text::TODO_SAMPLE)?;

    println!("Folder and file created successfully at: {}", file_path);

    Ok(())
}

pub fn git_init(folder_path: &str) -> std::io::Result<()> {
    let output = Command::new("git")
        .arg("init")
        .current_dir(folder_path)
        .output()?;

    if output.status.success() {
        println!("Git repository initialized successfully.");
    } else {
        println!("Failed to initialize Git repository.");
    }

    Ok(())
}

pub fn create_gitignore(folder_path: &str) -> std::io::Result<()> {
    let file_path = format!("./{}/.gitignore", folder_path);

    let mut file = File::create(&file_path)?;

    file.write_all(b"venv\n")?;
    file.write_all(b"__pycache__\n")?;
    file.write_all(b"dist\n")?;
    file.write_all(b"build\n")?;
    file.write_all(b"TODO.md\n")?;

    println!("Gitignore file created successfully at: {}", file_path);
    Ok(())
}

pub fn create_project_toml(config: &ProjectConfig, path: &str) -> std::io::Result<()> {
    let toml_content =
        toml::to_string(&config).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let file_path = format!("./{}/project.toml", path);
    fs::write(file_path, toml_content)?;
    Ok(())
}
