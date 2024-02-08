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

    pub fn load_info(info_file: &std::path::PathBuf) -> Result<ModInfo, std::io::Error> {
        let info = std::fs::read_to_string(info_file)?;
        let info: ModInfo = serde_json::from_str(&info)?;

        Ok(info)
    }
}