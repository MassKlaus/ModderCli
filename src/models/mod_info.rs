use serde::{Deserialize, Serialize};



#[derive(Debug, Deserialize, Serialize)]
pub struct ModInfo {
    pub name: String,
    pub description: String,
    pub version: String,
    pub current_branch: String,
}

impl ModInfo {
    pub fn new(name: String, description: String, version: String, current_branch: String) -> ModInfo {
        ModInfo {
            name,
            description,
            version,
            current_branch,
        }
    }
}