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

    pub fn load_branches(branchesFile: &PathBuf) -> Vec<Branch> {
        let branches = std::fs::read_to_string(branchesFile).unwrap();
        let branches: Vec<Branch> = serde_json::from_str(&branches).unwrap();

        branches
    }

    pub fn save_branches(branchesFile: &PathBuf, branches: Vec<Branch>) {
        let json = serde_json::to_string_pretty(&branches).unwrap();
        _ = std::fs::write(branchesFile, json);
    }
    
}