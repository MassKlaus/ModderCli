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
}