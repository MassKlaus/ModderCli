use std::path::PathBuf;

use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Branch {
    pub name: String,
    pub description: String,
    pub version: i32,
}

impl Branch {
    pub fn new(name: String, description: String, version: i32) -> Branch {
        Branch {
            name,
            description,
            version,
        }
    }

    pub fn load_branches(branchesFile: &PathBuf) -> Result<Vec<Branch>, std::io::Error> {
        let branches = std::fs::read_to_string(branchesFile)?;
        let branches: Vec<Branch> = serde_json::from_str(&branches)?;

        Ok(branches)
    }

    pub fn save_branches(branchesFile: &PathBuf, branches: Vec<Branch>) -> Result<(), std::io::Error>{
        let json = serde_json::to_string_pretty(&branches)?;
        _ = std::fs::write(branchesFile, json)?;

        Ok(())
    }
    
}