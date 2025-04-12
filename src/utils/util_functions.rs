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

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use super::*;

    #[test]
    fn test_create_folder_and_file() {
        
        let folder_path = "test_folder";
        // Ensure the directory is clean
        if PathBuf::from(folder_path).exists() {
            fs::remove_dir_all(folder_path).unwrap();
        }
        fs::create_dir(folder_path).unwrap();
        let result = create_folder_and_file(folder_path);
        assert!(result.is_ok());

        // Check if the folder and files were created
        assert!(PathBuf::from(folder_path).exists());
        assert!(PathBuf::from(folder_path).join("src").exists());
        assert!(PathBuf::from(folder_path)
            .join("src")
            .join("main.py")
            .exists());
        assert!(PathBuf::from(folder_path).join("TODO.md").exists());

        // Clean up
        fs::remove_dir_all(folder_path).unwrap();
    }

    #[test]
    fn test_git_init() {

        let folder_path = "test_gitinit_folder";

        // Ensure the directory is clean
        if PathBuf::from(folder_path).exists() {
            fs::remove_dir_all(folder_path).unwrap();
        }

        fs::create_dir(folder_path).unwrap();
        let result = git_init(folder_path);
        assert!(result.is_ok(),  "Failed to init git: {:?}", result);

        // Check if the .git directory was created
        assert!(PathBuf::from(folder_path).join(".git").exists());

        // Clean up
        fs::remove_dir_all(folder_path).unwrap();
    }

    #[test]
    fn test_create_gitignore() {

        let folder_path = "test_gitignore_folder";

        // Ensure the directory is clean
        if PathBuf::from(folder_path).exists() {
            fs::remove_dir_all(folder_path).unwrap();
        }
        fs::create_dir(folder_path).unwrap();

        let result = create_gitignore(folder_path);
        assert!(result.is_ok(), "Failed to create .gitignore: {:?}", result);

        // Check if the .gitignore file was created
        assert!(
            PathBuf::from(folder_path).join(".gitignore").exists(),
            ".gitignore file was not created"
        );

        // Clean up
        fs::remove_dir_all(folder_path).unwrap();
    }
    #[test]
    fn test_project_toml() {

        // Create a sample Config object
        let config = ProjectConfig {
            metadata: crate::models::models::ProjectMetaData {
                name: "test_project".to_string(),
                version: "0.1.0".to_string(),
                authors: vec!["Author".to_string()],
                license: "MIT".to_string(),
                description: "Test project".to_string(),
                python_version: "3.8".to_string(),
            },
            tool: crate::models::models::ToolSettings {
                package_manager: crate::models::models::PackageManager::Pip,
                conda_env_name: None,
            },
        };
    
        let folder_path = "test_project_toml_folder";
        // Ensure the directory is clean
        if PathBuf::from(folder_path).exists() {
            fs::remove_dir_all(folder_path).unwrap();
        }
        // Create the directory
        fs::create_dir(folder_path).unwrap();
        let result = create_project_toml(&config, folder_path);
        assert!(result.is_ok(), "Failed to create conda.toml: {:?}", result);
    
        // Check if the project.toml file was created
        assert!(PathBuf::from(folder_path).join("project.toml").exists());
        // Check if the contents of the file are correct
        let contents = fs::read_to_string(PathBuf::from(folder_path).join("project.toml")).unwrap();
        
        assert!(contents.contains("[metadata]"));
        assert!(contents.contains("name = \"test_project\""));
        assert!(contents.contains("version = \"0.1.0\""));
        assert!(contents.contains("authors = [\"Author\"]"));
        assert!(contents.contains("license = \"MIT\""));
        assert!(contents.contains("description = \"Test project\""));
        assert!(contents.contains("python_version = \"3.8\""));
        assert!(contents.contains("[tool]"));
        assert!(contents.contains("package_manager = \"Pip\""));
    
        // Clean up
        fs::remove_dir_all(folder_path).unwrap();


    }

    #[test]
    fn test_conda_toml() {

        // Create a sample Config object
        let config = ProjectConfig {
            metadata: crate::models::models::ProjectMetaData {
                name: "test_project".to_string(),
                version: "0.1.0".to_string(),
                authors: vec!["Author".to_string()],
                license: "MIT".to_string(),
                description: "Test project".to_string(),
                python_version: "3.8".to_string(),
            },
            tool: crate::models::models::ToolSettings {
                package_manager: crate::models::models::PackageManager::Conda,
                conda_env_name: Some("test_env".to_string()),
            },
        };

        let folder_path = "test_conda_toml_folder";
        // Ensure the directory is clean
        if PathBuf::from(folder_path).exists() {
            fs::remove_dir_all(folder_path).unwrap();
        }
        // Create the directory
        fs::create_dir(folder_path).unwrap();
        let result = create_project_toml(&config, folder_path);
        assert!(result.is_ok(), "Failed to create conda.toml: {:?}", result);

        // Check if the conda.toml file was created
        assert!(PathBuf::from(folder_path).join("project.toml").exists());
        // Check if the contents of the file are correct
        let contents = fs::read_to_string(PathBuf::from(folder_path).join("project.toml")).unwrap();

        assert!(contents.contains("[metadata]"));
        assert!(contents.contains("name = \"test_project\""));
        assert!(contents.contains("version = \"0.1.0\""));
        assert!(contents.contains("authors = [\"Author\"]"));
        assert!(contents.contains("license = \"MIT\""));
        assert!(contents.contains("description = \"Test project\""));
        assert!(contents.contains("python_version = \"3.8\""));
        assert!(contents.contains("[tool]"));
        assert!(contents.contains("package_manager = \"Conda\""));
        assert!(contents.contains("conda_env_name = \"test_env\""));

        // Clean up
        fs::remove_dir_all(folder_path).unwrap();
    }
}
