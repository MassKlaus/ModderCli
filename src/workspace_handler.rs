use std::path::PathBuf;

use crate::{branch::Branch, mod_info::ModInfo};

// make custom error for empty branch folder
#[derive(Debug)]
pub struct Workspace {
    pub root_folder: PathBuf,
    pub info: ModInfo,
    pub branches: Vec<Branch>,
}

pub enum SwitchResult {
    Success,
    NoFileMove
}

impl Workspace {
    pub fn new(root_folder: PathBuf, info: ModInfo, branches: Vec<Branch>) -> Workspace {
        Workspace {
            root_folder,
            info,
            branches,
        }
    }

    pub fn load_workspace() -> Result<Workspace, std::io::Error> {
        // Load the workspace
        println!("Load workspace");

        let root_folder = Workspace::find_root_folder()?;

        if root_folder == None {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Workspace not found.",
            ));
        }

        let root_folder = root_folder.unwrap();

        let info_file = root_folder.join(".info");
        let info = ModInfo::load_info(&info_file)?;

        let branch_file = root_folder.join("branches/.branches");
        let branches = Workspace::load_branches(&branch_file)?;

        Ok(Workspace {
            root_folder,
            info,
            branches,
        })
    }

    fn load_branches(branch_file: &PathBuf) -> Result<Vec<Branch>, std::io::Error> {
        let branches = std::fs::read_to_string(&branch_file)?;
        let branches = serde_json::from_str(&branches)?;

        Ok(branches)
    }

    pub fn find_root_folder() -> Result<Option<PathBuf>, std::io::Error> {
        // Find the root folder of the project
        let mut path = std::env::current_dir()?;
        let mut found = false;
        let mut count = 0;

        // limit it to 10 levels deep
        while !found && count < 10 {
            let dir = std::fs::read_dir(&path)?;

            for entry in dir {
                let entry = entry?;

                if entry.file_name() == ".info" {
                    found = true;
                    break;
                }
            }

            if !found {
                path = match path.parent() {
                    Some(p) => p.to_path_buf(),
                    None => break,
                };
            }

            count += 1;
        }

        return if found { Ok(Some(path)) } else { Ok(None) };
    }

    pub fn init(root_folder: PathBuf, info: ModInfo) -> Result<Workspace, std::io::Error> {
        // Create a new workspace in the current directory
        println!("Init");

        // check folder exists
        if !root_folder.exists() {
            std::fs::create_dir_all(&root_folder)?;
        } else {
            // check if the folder is empty
            let dir = std::fs::read_dir(&root_folder)?;

            // check if .info file exists
            for entry in dir {
                let entry = entry?;

                if entry.file_name() == ".info" {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::AlreadyExists,
                        "Workspace already exists.",
                    ));
                }
            }
        }

        let src_folder = root_folder.join("src");
        // Create a new directory for src, branches, and publish
        _ = std::fs::create_dir(src_folder);

        let publish_folder = root_folder.join("publish");
        _ = std::fs::create_dir(publish_folder);

        let branches_folder = root_folder.join("branches");
        _ = std::fs::create_dir(&branches_folder);

        let main_branch_folder = branches_folder.join("main");
        _ = std::fs::create_dir(main_branch_folder);

        // Create a file to store the branches
        let branch = Branch::new("main".to_string(), "Main branch".to_string(), 1);
        let branches = vec![];

        let mut workspace = Workspace::new(root_folder, info, branches);
        workspace.add_branch(branch)?;
        workspace.save()?;

        Ok(workspace)
    }

    pub fn src_folder_path(&self) -> PathBuf {
        self.root_folder.join("src")
    }

    pub fn info_path(&self) -> PathBuf {
        self.root_folder.join(".info")
    }

    pub fn branches_folder_path(&self) -> PathBuf {
        self.root_folder.join("branches")
    }

    pub fn branches_path(&self) -> PathBuf {
        self.root_folder.join("branches/.branches")
    }

    pub fn current_branch_folder_path(&self) -> Result<PathBuf, std::io::Error> {
        let current_branch = match &self.info.current_branch {
            Some(b) => b,
            None => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "No Branches Selected.",
                ))
            }
        };

        Ok(self.root_folder.join("branches").join(current_branch))
    }

    pub fn add_branch(&mut self, branch: Branch) -> Result<(), std::io::Error> {
        // check if branch already exists

        for b in &self.branches {
            if b.name == branch.name {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::AlreadyExists,
                    "Branch already exists.",
                ));
            }
        }

        // create the branch folder
        let branch_folder = self.branches_folder_path().join(&branch.name);
        _ = std::fs::create_dir(&branch_folder);

        self.branches.push(branch);

        Ok(())
    }

    pub fn switch_branch(&mut self, name: &str) -> Result<SwitchResult, std::io::Error> {
        // check if branch exists
        let found = self.branches.iter().find(|b| b.name == name);

        let branch = match found {
            Some(b) => b,
            None => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Branch not found.",
                ))
            }
        };

        self.info.current_branch = Some(branch.name.clone());

        // copy the branch files to the src folder
        let branch_folder = self.branches_folder_path().join(&branch.name);

        // get the latest version of the branch
        let latest_version = branch.version - 1;

        let version_string = format!("{}", latest_version);

        let version_folder = branch_folder.join(&version_string);

        // check if the version folder exists
        if !version_folder.exists() {
            return Ok(SwitchResult::NoFileMove);
        }

        let src_folder = self.src_folder_path();

        // check if the branch folder is empty
        let dir = std::fs::read_dir(&version_folder)?;

        for entry in dir {
            let entry = entry?;

            if entry.file_type()?.is_file() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Branch Folder Empty, No files removed or moved.",
                ));
            }
        }

        Workspace::recurcive_copy(&version_folder, &src_folder)?;

        Ok(SwitchResult::Success)
    }

    pub fn remove_branch_by_name(&mut self, name: &str) -> Result<(), std::io::Error> {
        // check if branch exists
        let found = self.branches.iter().find(|b| b.name == name);

        let branch = match found {
            Some(b) => b,
            None => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Branch not found.",
                ))
            }
        };

        self.info.current_branch = None;

        // remove the branch folder
        let branch_folder = self.branches_folder_path().join(&branch.name);
        _ = std::fs::remove_dir_all(&branch_folder);

        // remove the branch from the branches list
        self.branches.retain(|b| b.name != name);

        Ok(())
    }

    pub fn save_info(&self) -> Result<(), std::io::Error> {
        let json = serde_json::to_string_pretty(&self.info)?;
        _ = std::fs::write(self.info_path(), json)?;

        Ok(())
    }

    pub fn save_branches(&self) -> Result<(), std::io::Error> {
        let json = serde_json::to_string_pretty(&self.branches)?;
        _ = std::fs::write(self.branches_path(), json)?;

        Ok(())
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        self.save_info()?;
        self.save_branches()?;

        Ok(())
    }

    pub fn save_current_state(&mut self) -> Result<(), std::io::Error> {
        // Save the current state of the mod to the current branch
        let src_folder = self.src_folder_path();
        let current_branch_folder = match self.current_branch_folder_path() {
            Ok(p) => p,
            Err(e) => return Err(e),
        };

        let mut branch = None;

        let current_branch = match &self.info.current_branch {
            Some(b) => b,
            None => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "No Branches Selected",
                ))
            }
        };

        for b in &mut self.branches {
            if b.name.eq(current_branch) {
                branch = Some(b);
                break;
            }
        }

        let branch = match branch {
            Some(b) => b,
            None => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Current Branch does not exist in branches.",
                ))
            }
        };

        let version_string = format!("{}", branch.version);
        let new_branch_folder = current_branch_folder.join(&version_string);

        branch.version += 1;

        // ensure the branch folder exists
        _ = std::fs::create_dir_all(&new_branch_folder);

        if self.info.top_files_only {
            Workspace::top_level_copy(&src_folder, &new_branch_folder)
        } else {
            Workspace::recurcive_copy(&src_folder, &new_branch_folder)
        }
    }

    fn top_level_copy(src: &PathBuf, dest: &PathBuf) -> Result<(), std::io::Error> {
        // copy the files in the src folder to the branch folder
        let srcFiles = std::fs::read_dir(&src)?;

        for file in srcFiles {
            let file = file?;
            let file_name = file.file_name();
            let file_path = file.path();

            let new_file_path = dest.join(file_name);

            if file_path.is_file() {
                _ = std::fs::copy(file_path, new_file_path);
            }
        }

        Ok(())
    }

    fn recurcive_copy(src: &PathBuf, dest: &PathBuf) -> Result<(), std::io::Error> {
        // copy the files in the src folder to the branch folder
        let srcFiles = std::fs::read_dir(&src)?;

        for file in srcFiles {
            let file = file?;
            let file_name = file.file_name();
            let file_path = file.path();

            let new_file_path = dest.join(file_name);

            if file_path.is_dir() {
                _ = std::fs::create_dir(&new_file_path);
                Workspace::recurcive_copy(&file_path, &new_file_path)?;
            } else {
                _ = std::fs::copy(file_path, new_file_path);
            }
        }

        Ok(())
    }
}
