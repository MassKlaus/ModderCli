use std::{
    env,
    path::{Path, PathBuf},
};

use crate::{
    file_filter::FileFilter,
    models::{branch::Branch, mod_info::ModInfo},
};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
#[error("FileHandler Error: {0}")]
pub enum Error {
    Io(#[from] std::io::Error),
    Serde(#[from] serde_json::Error),
    Glob(#[from] globset::Error),
}

pub enum CopyResult {
    Ok,
    NoFileToCopy,
}

#[derive(Debug)]
pub struct FileHandler {
    root: PathBuf,
    filter: FileFilter,
}

impl FileHandler {
    pub fn new(root: PathBuf) -> Self {
        let filter = FileFilter::new(vec![]).unwrap();
        FileHandler { root, filter }
    }

    const INFO_FILE: &'static str = ".info";
    const BRANCHES_DIR: &'static str = "branches";
    const SRC_DIR: &'static str = "src";
    const ASSETS_DIR: &'static str = "assets";
    const PUBLISH_DIR: &'static str = "publish";

    const BRANCH_FILE: &'static str = ".branch";
    const IGNORE_FILE: &'static str = ".ignore";

    pub fn find_root() -> Result<Option<PathBuf>> {
        let mut current: &Path = &env::current_dir()?;

        let mut max_depth = 10;

        while max_depth > 0 {
            if current.join(FileHandler::INFO_FILE).exists() {
                return Ok(Some(current.to_path_buf()));
            }

            current = match current.parent() {
                Some(parent) => parent,
                None => break,
            };

            max_depth -= 1;
        }

        Ok(None)
    }

    pub fn create_workspace(&self, mod_info: &ModInfo, branches: &Vec<Branch>) -> Result<()> {
        std::fs::create_dir_all(&self.root)?;
        std::fs::create_dir_all(self.get_branches())?;
        std::fs::create_dir_all(self.get_src())?;
        std::fs::create_dir_all(self.get_assets())?;
        std::fs::create_dir_all(self.get_publish())?;

        self.save_info(mod_info)?;
        self.save_branches(branches)?;
        self.save_ignore(&[])?;

        // create the branches
        for branch in branches {
            self.create_branch(&branch)?;
        }

        Ok(())
    }

    pub fn get_info(&self) -> PathBuf {
        self.root.join(FileHandler::INFO_FILE)
    }

    pub fn get_branches(&self) -> PathBuf {
        self.root.join(FileHandler::BRANCHES_DIR)
    }

    pub fn get_branch_folder(&self, branch: &Branch) -> PathBuf {
        self.get_branches().join(&branch.name)
    }

    pub fn create_branch(&self, branch: &Branch) -> Result<()> {
        std::fs::create_dir_all(self.get_branch_folder(branch))?;

        Ok(())
    }

    pub fn get_src(&self) -> PathBuf {
        self.root.join(FileHandler::SRC_DIR)
    }

    pub fn get_assets(&self) -> PathBuf {
        self.root.join(FileHandler::ASSETS_DIR)
    }

    pub fn get_publish(&self) -> PathBuf {
        self.root.join(FileHandler::PUBLISH_DIR)
    }

    pub fn get_branch_file(&self) -> PathBuf {
        self.get_branches().join(FileHandler::BRANCH_FILE)
    }

    pub fn get_ignore_file(&self) -> PathBuf {
        self.root.join(FileHandler::IGNORE_FILE)
    }

    pub fn load_branches(&self) -> Result<Vec<Branch>> {
        let branch_file = self.get_branch_file();

        if !branch_file.exists() {
            return Ok(vec![]);
        }

        let file = std::fs::read_to_string(branch_file)?;
        let branches: Vec<Branch> = serde_json::from_str(&file)?;

        Ok(branches)
    }

    pub fn save_branches(&self, branches: &Vec<Branch>) -> Result<()> {
        let branch_file = self.get_branch_file();
        let file = serde_json::to_string_pretty(&branches)?;

        std::fs::write(branch_file, file)?;

        Ok(())
    }

    pub fn save_info(&self, info: &ModInfo) -> Result<()> {
        let info_file = self.get_info();

        let info = serde_json::to_string_pretty(info)?;
        std::fs::write(info_file, info)?;

        Ok(())
    }

    pub fn load_info(&self) -> Result<ModInfo> {
        let info_file = self.get_info();

        let info = std::fs::read_to_string(info_file)?;
        let info: ModInfo = serde_json::from_str(&info)?;

        Ok(info)
    }

    pub fn load_ignore(&self) -> Result<Vec<String>> {
        let ignore_file = self.get_ignore_file();

        if !ignore_file.exists() {
            return Ok(vec![]);
        }

        let file = std::fs::read_to_string(ignore_file)?;
        let ignore: Vec<String> = file.lines().map(|s| s.to_string()).collect();

        Ok(ignore)
    }

    pub fn init_filter(&mut self, patterns: Vec<String>) -> Result<()> {
        let filter = FileFilter::new(patterns)?;

        self.filter = filter;

        Ok(())
    }

    pub fn save_ignore(&self, ignore: &[String]) -> Result<()> {
        let ignore_file = self.get_ignore_file();
        let file = ignore.join("\n");

        std::fs::write(ignore_file, file)?;

        Ok(())
    }

    pub fn filter_copy_src(&self, to: &Path) -> Result<()> {
        self.filter_copy_folder(&self.get_src(), to)
    }

    pub fn filter_copy_to_src(&self, from: &Path) -> Result<()> {
        self.remove_folder_content(&self.get_src())?;
        self.filter_copy_folder(from, &self.get_src())
    }

    pub fn remove_folder_content(&self, folder: &Path) -> Result<()> {
        if !folder.exists() {
            return Ok(());
        }

        for entry in std::fs::read_dir(folder)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                std::fs::remove_dir_all(&path)?;
            } else {
                std::fs::remove_file(&path)?;
            }
        }

        Ok(())
    }

    fn filter_copy_folder(&self, from: &Path, to: &Path) -> Result<()> {
        if !from.exists() {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "From Folder not found",
            )));
        }

        let files = Self::recurcive_file_explore(from)?;
        let files = self.filter_files(files, from);

        std::fs::create_dir_all(to)?;

        for file in files {
            let relative = file.strip_prefix(from).unwrap();
            let to = to.join(relative);

            std::fs::create_dir_all(to.parent().unwrap())?;
            std::fs::copy(&file, to)?;
        }

        Ok(())
    }

    fn recurcive_file_explore(path: &Path) -> Result<Vec<PathBuf>> {
        let mut files = vec![];

        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let mut sub_files = FileHandler::recurcive_file_explore(&path)?;
                files.append(&mut sub_files);
            } else {
                files.push(path);
            }
        }

        Ok(files)
    }

    fn filter_files(&self, files: Vec<PathBuf>, root: &Path) -> Vec<PathBuf> {
        files
            .into_iter()
            .filter(|file| self.filter.allow_file(file, root))
            .collect()
    }

    pub fn remove_branch(&self, branch_path: &PathBuf) -> Result<()> {
        std::fs::remove_dir_all(branch_path)?;

        Ok(())
    }
}
