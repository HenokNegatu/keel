use serde::{Deserialize, Serialize};
use clap::{Parser, Subcommand};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum PackageManager {
    Pip,
    Conda,
}

impl PackageManager {
    pub fn from_str(s: &str) -> Option<PackageManager> {
        match s {
            "pip" => Some(PackageManager::Pip),
            "conda" => Some(PackageManager::Conda),
            _ => None,
        }
    }
}


impl Default for PackageManager {
    fn default() -> Self {
        PackageManager::Pip  // Default to pip
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub metadata: ProjectMetaData,
    pub tool: ToolSettings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectMetaData {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    pub license: String,
    pub description: String,
    pub python_version: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ToolSettings {
    #[serde(default)]
    pub package_manager: PackageManager,
    pub conda_env_name: Option<String>,
}

#[derive(Parser)]
#[command(name = "keel", author = "Henok Negatu", version = "v0.1.0-alpha.1", about = "Keel is a cli tool for creating python projects", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
 New,
 Activate,
 Deactivate   
}
