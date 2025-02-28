#[derive(Debug)]
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

pub struct Project {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    pub license: String,
    pub python_version: String,
    pub conda_env_name: Option<String>, // optional
    pub package_manager: PackageManager,
}
