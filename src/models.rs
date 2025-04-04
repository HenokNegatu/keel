use serde::Serialize;

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
pub struct ProjectConfig {
    pub project: ProjectMetaData,
    pub tool: ToolSettings,
}

#[derive(Debug, Serialize)]
pub struct ProjectMetaData {
    pub metadata: Project,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct ToolSettings {
    pub package_manager: PackageManager,
}
