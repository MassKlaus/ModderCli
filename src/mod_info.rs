use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ModInfo {
    pub name: String,
    pub author: String,
    pub description: String,
    pub current_branch: Option<String>,
    pub top_files_only: bool,
    pub file_type: Option<String>,
}

impl ModInfo {
    pub fn new(name: String, author: String, description: String, current_branch: Option<String>) -> ModInfo {
        ModInfo {
            name,
            author,
            description,
            current_branch,
            top_files_only: false,
            file_type: None,
        }
    }

    pub fn load_info(info_file: &std::path::PathBuf) -> Result<ModInfo, std::io::Error> {
        let info = std::fs::read_to_string(info_file)?;
        let info: ModInfo = serde_json::from_str(&info)?;

        Ok(info)
    }

    
}