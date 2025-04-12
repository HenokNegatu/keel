#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_create_folder_and_file() {
        use crate::utils::util_functions::create_folder_and_file;
        
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
        use crate::utils::util_functions::git_init;

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
        use crate::utils::util_functions::create_gitignore;

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
        use crate::models::models::ProjectConfig;
        use crate::utils::util_functions::create_project_toml;

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
        use crate::models::models::ProjectConfig;
        use crate::utils::util_functions::create_project_toml;

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
