use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct ModInfo {
    pub name: String,
    pub author: String,
    pub description: String,
    pub current_branch: String,
}

impl ModInfo {
    pub fn new(name: String, author: String, description: String, current_branch: String) -> ModInfo {
        ModInfo {
            name,
            author,
            description,
            current_branch,
        }
    }
}