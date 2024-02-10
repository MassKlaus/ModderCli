use crate::models::{branch::Branch, mod_info::ModInfo};


#[derive(Debug)]
pub struct Workspace {
    pub mod_info: ModInfo,
    pub branches: Vec<Branch>,
}

#[derive(Debug, thiserror::Error)]
#[error("Workspace Error: {0}")]
pub enum WorkspaceError {
    BranchNotFound(String),
}

type Result<T> = std::result::Result<T, WorkspaceError>;

impl Workspace {
    pub fn new(mod_info: ModInfo, branches: Vec<Branch>) -> Workspace {
        Workspace {
            mod_info,
            branches,
        }
    }

    pub fn get_branch(&self, name: &str) -> Option<&Branch> {
        self.branches.iter().find(|b| b.name == name)
    }

    pub fn add_branch(&mut self, branch: Branch) -> Result<()> {
        if self.branches.iter().any(|b| b.name == branch.name) {
            return Err(WorkspaceError::BranchNotFound(format!("Branch {} already exists", branch.name)));
        }

        self.branches.push(branch);

        Ok(())
    }

    pub fn change_branch_version(&mut self, branch: &str, version: i32) -> Result<()> {
        if let Some(b) = self.branches.iter_mut().find(|b| b.name == branch) {
            b.version = version;
            Ok(())
        } else {
            Err(WorkspaceError::BranchNotFound(branch.to_string()))
        }
    }
}