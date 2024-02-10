use serde::{Deserialize, Serialize};



#[derive(Debug, Deserialize, Serialize)]
pub struct Branch {
    pub name: String,
    pub version: i32,
}

impl Branch {
    pub fn new(name: String, version: i32) -> Branch {
        Branch {
            name,
            version,
        }
    }
}

impl PartialEq for Branch {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}