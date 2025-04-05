use serde::{Deserialize, Serialize};
use clap::{Parser, Subcommand};

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    pub license: String,
    pub python_version: String,
    pub conda_env_name: Option<String>,
    #[serde(skip_serializing)]
    pub package_manager: PackageManager,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub project: ProjectMetaData,
    pub tool: ToolSettings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectMetaData {
    pub metadata: Project,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolSettings {
    pub package_manager: PackageManager,
}

#[derive(Parser)]
#[command(name = "keel")]
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
