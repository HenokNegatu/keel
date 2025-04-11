use colored::Colorize;

use crate::models::models::ProjectConfig;
use crate::utils::sample_text;
use std::fs::{self, File};
use std::io::Write;
use std::process::Command;

use crate::models::error::UtilFuncError;
use std::path::PathBuf;

pub fn create_folder_and_file(folder_path: &str) -> Result<(), UtilFuncError> {
    let src_folder_path = PathBuf::from(folder_path).join("src");
    let asset_path = PathBuf::from(folder_path).join("assets");
    let file_path = src_folder_path.join("main.py");
    let todo_file_path = PathBuf::from(folder_path).join("TODO.md");

    fs::create_dir_all(&src_folder_path).map_err(|e| UtilFuncError::CreateDirError {
        path: src_folder_path.display().to_string(),
        source: e,
    })?;
    fs::create_dir_all(&asset_path).map_err(|e| UtilFuncError::CreateDirError {
        path: src_folder_path.display().to_string(),
        source: e,
    })?;

    let mut file = File::create(&file_path).map_err(|e| UtilFuncError::CreateFileError {
        path: file_path.display().to_string(),
        source: e,
    })?;

    writeln!(file, "{}", sample_text::PYTHON_CODE).map_err(|e| UtilFuncError::WriteFileError {
        path: file_path.display().to_string(),
        source: e,
    })?;

    let mut todo_file =
        File::create(&todo_file_path).map_err(|e| UtilFuncError::CreateFileError {
            path: file_path.display().to_string(),
            source: e,
        })?;
    writeln!(todo_file, "{}", sample_text::TODO_SAMPLE).map_err(|e| {
        UtilFuncError::WriteFileError {
            path: todo_file_path.display().to_string(),
            source: e,
        }
    })?;

    println!(
        "{} {}", "Folder and file created successfully at:".dimmed(),
        folder_path.dimmed()
    );

    Ok(())
}

pub fn git_init(folder_path: &str) -> Result<(), UtilFuncError> {
    Command::new("git")
        .arg("init")
        .current_dir(folder_path)
        .output()?;
    println!("{}", "initialized git".dimmed());
    Ok(())
}

pub fn create_gitignore(folder_path: &str) -> Result<(), UtilFuncError> {
    let file_path = format!("./{}/.gitignore", folder_path);

    let mut file = File::create(&file_path)?;

    file.write_all(b"venv\n")?;
    file.write_all(b"__pycache__\n")?;
    file.write_all(b"dist\n")?;
    file.write_all(b"build\n")?;
    file.write_all(b"TODO.md\n")?;

    println!("{} {}","Gitignore file created successfully at:".dimmed(), file_path.dimmed());
    Ok(())
}

pub fn create_project_toml(config: &ProjectConfig, path: &str) -> Result<(), UtilFuncError> {
    let toml_content = toml::to_string(&config)?;
    let file_path = format!("./{}/project.toml", path);
    fs::write(file_path, toml_content)?;
    println!(
        "{} {}",
        "Project.toml file created successfully at:".dimmed(),
        path.dimmed()
    );
    Ok(())
}
