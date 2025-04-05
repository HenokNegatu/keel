use core::panic;
use crate::utils::util_functions::{create_folder_and_file, create_gitignore, git_init};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use crate::commands::executor::create_venv;
use crate::utils::util_functions::create_project_toml;
use crate::models::models::{PackageManager, Project, ProjectConfig, ProjectMetaData, ToolSettings};

pub fn new() {

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

let selections = &["pip", "conda"];
let package_manager_input = Select::with_theme(&ColorfulTheme::default())
    .with_prompt("Choose project type")
    .items(selections)
    .interact()
    .unwrap();

let package_manager =
    if let Some(pm) = PackageManager::from_str(selections[package_manager_input]) {
        pm
    } else {
        panic!("Invalid input. Please select 'pip' or 'conda'.");
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

let project = Project {
    name,
    version,
    authors,
    license,
    python_version,
    conda_env_name,
    package_manager,
};

println!("\nProject created successfully!");
println!("Name: {}", project.name);
println!("Version: {}", project.version);
println!("Authors: {:?}", project.authors);
println!("License: {}", project.license);
println!("Python Version: {}", project.python_version);

create_venv(
    &project.name,
    &project.package_manager,
    &project.conda_env_name,
);

match create_folder_and_file(project.name.as_str()) {
    Ok(_) => (),
    Err(err) => println!("Error creating folder and file: {}", err),
}

match git_init(project.name.as_str()) {
    Ok(_) => match create_gitignore(project.name.as_str()) {
        Ok(_) => (),
        Err(err) => println!("Error creating gitignore: {}", err),
    },
    Err(err) => println!("Error initializing Git: {}", err),
}

let config = ProjectConfig {
    project: ProjectMetaData {
        metadata: Project {
            name: project.name,
            version: project.version,
            authors: project.authors,
            license: project.license,
            python_version: project.python_version,
            conda_env_name: project.conda_env_name,
            package_manager: PackageManager::Pip, //useless must be omitted
        },
        description: "A Python project".to_string(),
    },
    tool: ToolSettings {
        package_manager: project.package_manager,
    },
};
match create_project_toml(&config, &config.project.metadata.name.as_str()) {
    Ok(_) => {}
    Err(e) => eprintln!("{}", e),
};

if let Some(env) = &config.project.metadata.conda_env_name {
    println!("Conda Env: {}", env);
}
println!(
    "Package Manager: {}",
    match config.project.metadata.package_manager {
        PackageManager::Pip => "Pip",
        PackageManager::Conda => "Conda",
    }
);
}
