#![allow(non_snake_case)]
use std::path::PathBuf;

use clap::{Error, Parser};

mod args;
mod branch;
mod mod_info;

fn main() -> Result<(), Error>{
    let args = args::CliArgs::parse();

    let root_folder_option = FindRootFolder()?;

    if root_folder_option == None {
        match args.action_context {
            args::ActionContext::Init => {
                let current_folder = std::env::current_dir()?;
                Init(&current_folder)?;
            }
            _ => {
                println!("You need to initialize a workspace first.\nUse 'ModderCli -h' for help.");
            }
        }

        return Ok(());
    }

    let root_folder = root_folder_option.unwrap();
    let info_path = root_folder.join(".info");
    let mod_info = mod_info::ModInfo::load_info(&info_path)?;

    match args.action_context {
        args::ActionContext::Init => {
            println!("You have already initialized a workspace.");
        }
        args::ActionContext::Branch(branch) => match branch.action {
            args::branches::BranchAction::Switch(value) => {
                println!("Switch to branch: {}", value.value);
            }
            args::branches::BranchAction::Create(value) => _ = CreateBranch(&root_folder, &value.value),
            args::branches::BranchAction::Delete(value) => {
                println!("Delete branch: {}", value.value);
            }
            args::branches::BranchAction::List => {
                ListBranches(&root_folder)?;
            }
        },
        args::ActionContext::Save => {
            _ = SaveCurrentState(root_folder, mod_info.current_branch)?;
        }
    }

    Ok(())
}

fn FindRootFolder() -> Result<Option<PathBuf>, std::io::Error> {
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

fn getModInfoFromUser() -> mod_info::ModInfo {
    // Get the mod info from the user
    // Get Mod Name
    println!("Enter mod name: ");
    let mut name = String::new();
    std::io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    let name = name.trim().to_string();

    // Get mod author
    println!("Enter mod author: ");
    let mut author = String::new();
    std::io::stdin()
        .read_line(&mut author)
        .expect("Failed to read line");
    let author = author.trim().to_string();

    // Get mod description
    println!("Enter mod description: ");
    let mut description = String::new();
    std::io::stdin()
        .read_line(&mut description)
        .expect("Failed to read line");
    let description = description.trim().to_string();

    return mod_info::ModInfo::new(name, author, description, "main".to_string());
}

fn Init(root_folder: &PathBuf) -> Result<(), std::io::Error>{
    // Create a new workspace in the current directory
    println!("Init");

    let info = getModInfoFromUser();

    let info_file = root_folder.join(".info");
    // Create file .info to store info about the project workflow
    let json = serde_json::to_string_pretty(&info)?;
    _ = std::fs::write(info_file, json);

    let src_folder = root_folder.join("src");
    // Create a new directory for src, branches, and publish
    _ = std::fs::create_dir(src_folder);

    let publish_folder = root_folder.join("publish");
    _ = std::fs::create_dir(publish_folder);

    let branches_folder = root_folder.join("branches");
    _ = std::fs::create_dir("branches");

    let main_branch_folder = branches_folder.join("main");
    _ = std::fs::create_dir(main_branch_folder);

    let branches_file = branches_folder.join(".branches");

    // Create a file to store the branches
    let branch = branch::Branch::new("main".to_string(), "Main branch".to_string(), 1);

    let branches = vec![branch];
    branch::Branch::save_branches(&branches_file, branches)?;

    println!("Workspace initialized.");

    print!("Put your source files in the src folder and use 'ModderCli branch create <branch_name>' to create a new branch.");
    print!("Default branch is 'main'.");

    Ok(())
}

fn CreateBranch(root: &PathBuf, branchName: &str) -> Result<(), std::io::Error> {
    // Create a new branch
    println!("Create branch");

    // branch path concat
    let branchPath = root.join("branches");
    let branchFile = branchPath.join(".branches");
    let newBranchPath = branchPath.join(branchName);

    let branch = branch::Branch::new(branchName.to_string(), "New branch".to_string(), 1);

    // Load the branches file
    let mut branches = branch::Branch::load_branches(&branchFile)?;

    // check if branch already exists
    for b in &branches {
        if b.name == branchName {
            println!("Branch already exists.");
            return Ok(());
        }
    }

    // Create a new directory for the branch
    _ = std::fs::create_dir(newBranchPath);

    // Add the new branch to the branches file
    branches.push(branch);

    // Save the branches file
    branch::Branch::save_branches(&branchFile, branches)?;

    println!("Branch created.");
    Ok(())
}

fn ListBranches(root: &PathBuf) -> Result<(), std::io::Error> {
    // List all branches
    println!("Branches:");

    let branchFiles = root.join("branches").join(".branches");

    let branches = branch::Branch::load_branches(&branchFiles)?;

    for branch in &branches {
        println!(
            "• {} (v.{}): {}",
            branch.name, branch.version, branch.description
        );
    }

    Ok(())
}

fn SaveCurrentState(root: PathBuf, currentBranch: String) -> Result<(), std::io::Error> {
    // Save the current state of the mod to the current branch
    println!("Saving current state to branch: {}", currentBranch);

    let srcFolder = root.join("src");
    let branchFolder = root.join("branches").join(&currentBranch);

    // copy the files in the src folder to the branch folder
    let srcFiles = std::fs::read_dir(&srcFolder)?;

    for file in srcFiles {
        let file = file?;
        let file_name = file.file_name();
        let file_path = file.path();

        let new_file_path = branchFolder.join(file_name);

        _ = std::fs::copy(file_path, new_file_path);
    }

    println!("Current state saved to branch: {}", currentBranch);

    Ok(())
}
