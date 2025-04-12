use crate::commands::executor::create_venv;
use crate::models::models::{PackageManager, ProjectConfig, ProjectMetaData, ToolSettings};
use crate::utils::util_functions::create_project_toml;
use crate::utils::util_functions::{create_folder_and_file, create_gitignore, git_init};
use colored::Colorize;
use core::panic;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use color_eyre::eyre::{Result, WrapErr};

pub fn new() -> Result<()> {

    color_eyre::install()?;

    let name = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Project Name?")
        .default("myProject".to_string())
        .interact_text()
        .unwrap();

    let version = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Project Version?")
        .default("0.1.0".to_string())
        .interact_text()
        .unwrap();

    let authors_input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter authors (comma-separated)")
        .interact_text()
        .unwrap();

    let license: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Project license")
        .default("MIT".to_string())
        .interact_text()
        .unwrap();

    let python_version: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter Python version")
        .default("3.11".into()) // optional default
        .interact_text()
        .unwrap();

    let description = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Project description?(optional)")
        .default(" ".to_string())
        .interact_text()
        .unwrap();

    let selections = &["pip", "conda"];
    let package_manager_input = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose Package Manager")
        .items(selections)
        .interact()
        .unwrap();

    let package_manager =
        if let Some(pm) = PackageManager::from_str(selections[package_manager_input]) {
            pm
        } else {
            panic!("{}", "Invalid input. Please select 'pip' or 'conda'.".red());
        };

    let conda_env_name = match package_manager {
        PackageManager::Pip => None,
        PackageManager::Conda => Some(
            Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Conda environment Name?")
                .default(name.clone())
                .interact_text()
                .unwrap(),
        ),
    };

    let authors: Vec<String> = authors_input
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    let project = ProjectConfig {
        metadata: ProjectMetaData {
            name,
            version,
            authors,
            license,
            description,
            python_version,
        },
        tool: ToolSettings {
            package_manager,
            conda_env_name,
        },
    };

    create_venv(
        &project.metadata.name,
        &project.tool.package_manager,
        &project.tool.conda_env_name,
    ).wrap_err("Failed to create virtual env")?;

    create_folder_and_file(&project.metadata.name).wrap_err("Failed to create folder and file")?;

    git_init(project.metadata.name.as_str()).wrap_err("Error initializing Git")?;
    
    create_gitignore(project.metadata.name.as_str()).wrap_err("Error creating gitignore:")?;

    create_project_toml(&project, &project.metadata.name.as_str()).wrap_err("Error creating project.toml")?;

    if let Some(env) = &project.tool.conda_env_name {
        println!("Conda Env: {}", env);
    }

    println!(
        "\n{}: {}\n{}: {}\n{}: {:?}\n{}: {}\n{}: {}\n{}: {}",
        "Name".blue(),
        project.metadata.name,
        "Version".blue(),
        project.metadata.version,
        "Authors".blue(),
        project.metadata.authors,
        "License".blue(),
        project.metadata.license,
        "Python Version".blue(),
        project.metadata.python_version,
        "Package Manager".blue(),
        match project.tool.package_manager {
            PackageManager::Pip => "Pip",
            PackageManager::Conda => "Conda",
        }
    );
    Ok(())
}
