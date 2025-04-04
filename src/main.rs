mod models;
mod utils;
mod executor;

use executor::executor::create_venv;
use models::{PackageManager, Project};
use crate::utils::util_functions::{read_input, create_folder_and_file, git_init, create_gitignore};

fn main() {
    let name = read_input("Enter project name: ");
    let version = read_input("Enter version: ");
    let authors_input = read_input("Enter authors (comma-separated): ");
    let license = read_input("Enter license: ");
    let python_version = read_input("Enter Python version: ");
    let mut package_manager_input = read_input("Enter package manager (pip/conda): ");

    let package_manager = loop {
        if let Some(pm) = PackageManager::from_str(&package_manager_input) {
            break pm;
        } else {
            println!("Invalid input. Please enter 'pip' or 'conda'.");
            package_manager_input = read_input("Enter package manager (pip/conda): ");
        }
    };

    let conda_env_name = match package_manager {
        PackageManager::Pip => None,
        PackageManager::Conda => {
            let env_name = read_input(format!("Enter Conda environment name: (default: {})", name).as_str());
            if env_name.is_empty() {
                Some(name.clone())
            } else {
                Some(env_name)
            }
        }
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

    create_venv(&project.name, &project.package_manager, &project.conda_env_name);

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

    if let Some(env) = &project.conda_env_name {
        println!("Conda Env: {}", env);
    }
    println!(
        "Package Manager: {}",
        match project.package_manager {
            PackageManager::Pip => "Pip",
            PackageManager::Conda => "Conda",
        }
    );
}
