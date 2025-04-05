use crate::models::models::ProjectConfig;
use std::fs;

//not implemented yet
pub fn activate() {
    let config_str = fs::read_to_string("./project.toml").unwrap();
    let config: ProjectConfig = toml::from_str(config_str.as_str()).unwrap();
    println!("{:?}", config);
}