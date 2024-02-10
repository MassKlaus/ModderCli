use std::env;

use args::{
    branches::{BranchAction, SwitchBranch},
    ActionContext, CliArgs,
};
use clap::Parser;
use file_handler::FileHandler;
use models::{
    branch::Branch,
    mod_info::ModInfo,
};
use workspace::Workspace;

mod args;
mod file_filter;
mod file_handler;
mod models;
mod workspace;

static DEFAULT_BRANCH: &str = "main";

type Result<T> = std::result::Result<T, ExecError>;

#[derive(Debug, thiserror::Error)]
pub enum ExecError {
    #[error("Error: {0}")]
    Error(String),

    #[error("FileHandler Error: {0}")]
    FileHandler(#[from] file_handler::Error),

    #[error("Workspace Error: {0}")]
    Workspace(#[from] workspace::WorkspaceError),
}

fn main() {
    let args = CliArgs::parse();

    let root = match file_handler::FileHandler::find_root() {
        Ok(root) => root,
        Err(e) => {
            eprintln!("Error finding root: {}", e);
            return;
        }
    };

    let Some(root) = root else {
        // if init command then return current directory
        if let ActionContext::Init(command) = args.action_context {
            let mut root = env::current_dir().unwrap();

            if let Some(folder_name) = command.folder_name {
                root.push(folder_name);
            }

            let mod_info = get_user_mod_input();
            let file_manager = FileHandler::new(root);

            let res = file_manager
                .create_workspace(&mod_info, &vec![Branch::new(DEFAULT_BRANCH.to_string(), 0)]);

            if let Err(e) = res {
                eprintln!("Error creating workspace: {}", e);
            }
        } else {
            panic!("No mod found in current directory or any of its parents");
        }

        return;
    };

    let mut file_handler = file_handler::FileHandler::new(root);
    let mod_info = match file_handler.load_info() {
        Ok(info) => info,
        Err(e) => {
            panic!("Error loading mod info: {}", e);
        }
    };
    let branches = match file_handler.load_branches() {
        Ok(branches) => branches,
        Err(e) => {
            panic!("Error loading branches: {}", e);
        }
    };

    let pattens = match file_handler.load_ignore() {
        Ok(patterns) => patterns,
        Err(e) => {
            panic!("Error loading ignore patterns: {}", e);
        }
    };

    match file_handler.init_filter(pattens) {
        Ok(_) => {}
        Err(e) => {
            panic!("Error initializing file filter: {}", e);
        }
    }

    let mut workspace = Workspace::new(mod_info, branches);
    match handle_command(args.action_context, &mut workspace, &file_handler) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error handling command: {}", e);
        }
    }

    match file_handler.save_info(&workspace.mod_info) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error saving mod info: {}", e);
        }
    }

    match file_handler.save_branches(&workspace.branches) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error saving branches: {}", e);
        }
    }
}

fn handle_command(
    command: ActionContext,
    workspace: &mut Workspace,
    file_handler: &FileHandler,
) -> Result<()> {
    match command {
        ActionContext::Init(_) => {
            // Project already initialized
            print!("Project already initialized");
        }
        ActionContext::Save => {
            // Save the current state of the mod to the current branch
            let branch = workspace.mod_info.current_branch.clone();

            let (branch_path, new_version) = {
                let Some(branch) = workspace.get_branch(&branch) else {
                    return Err(ExecError::Error("Branch not found".to_string()));
                };

                (file_handler.get_branch_folder(branch), branch.version + 1)
            };

            workspace.change_branch_version(&branch, new_version)?;

            let branch_path = branch_path.join(new_version.to_string());

            file_handler.filter_copy_src(&branch_path)?;
        }
        ActionContext::Branch(command) => {
            handle_branch_action(command.action, workspace, file_handler)?
        }
        ActionContext::Restore(command) => {
            let branch = workspace.mod_info.current_branch.clone();

            let (branch_path, version) = {
                let Some(branch) = workspace.get_branch(&branch) else {
                    return Err(ExecError::Error("Branch not found".to_string()));
                };

                (file_handler.get_branch_folder(branch), branch.version)
            };

            let version = match command.version {
                Some(version) => version,
                None => version,
            };

            let version_path = branch_path.join(version.to_string());

            if version_path.exists() {
                file_handler.filter_copy_to_src(&version_path)?;
            }
            else {
                file_handler.remove_folder_content(&file_handler.get_src())?;
            }
        }
    }

    Ok(())
}

fn handle_branch_action(
    command: BranchAction,
    workspace: &mut Workspace,
    file_handler: &FileHandler,
) -> Result<()> {
    match command {
        BranchAction::Create(command) => {
            let branch = Branch::new(command.branch.clone(), 0);
            file_handler.create_branch(&branch)?;
            workspace.add_branch(branch)?;

            file_handler.save_info(&workspace.mod_info)?;
            file_handler.save_branches(&workspace.branches)?;

            if command.swap {
                workspace.mod_info.current_branch = command.branch.clone();

                let switch_branch = SwitchBranch {
                    branch: command.branch.clone(),
                };

                let branch_action = BranchAction::Switch(switch_branch);

                handle_branch_action(branch_action, workspace, file_handler)?;

                if command.save {
                    let save_command = ActionContext::Save;
                    handle_command(save_command, workspace, &file_handler)?
                }
            }
        }
        BranchAction::Switch(command) => {
            let branch = workspace
                .get_branch(&command.branch)
                .ok_or_else(|| ExecError::Error(format!("Branch {} not found", command.branch)))?;

            let branch_path = file_handler.get_branch_folder(branch);
            let version_path = branch_path.join(branch.version.to_string());

            if version_path.exists() {
                file_handler.filter_copy_to_src(&version_path)?;
            } else {
                println!("Branch Version doesn't exist");
            }

            workspace.mod_info.current_branch = command.branch.clone();

            file_handler.save_info(&workspace.mod_info)?;
        }
        BranchAction::Delete(command) => {
            if command.value == workspace.mod_info.current_branch {
                return Err(ExecError::Error("Cannot delete current branch".to_string()));
            }

            let branch = workspace
                .get_branch(&command.value)
                .ok_or_else(|| ExecError::Error(format!("Branch {} not found", command.value)))?;

            let branch_path = file_handler.get_branch_folder(branch);

            file_handler.remove_branch(&branch_path)?;

            workspace.branches.retain(|b| b.name != command.value);

            file_handler.save_info(&workspace.mod_info)?;
            file_handler.save_branches(&workspace.branches)?;
        }
        _ => {
            panic!("Command not implemented")
        }
    }

    Ok(())
}

fn get_user_mod_input() -> ModInfo {
    let name = get_user_input("Enter the name of the mod: ");
    let description = get_user_input("Enter the description of the mod: ");
    let version = get_user_input("Enter the version of the mod: ");

    ModInfo::new(name, description, version, DEFAULT_BRANCH.to_string())
}

fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
