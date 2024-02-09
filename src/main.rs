#![allow(non_snake_case)]
use args::{branches, ActionContext};
use clap::{error::Error, Parser};
use mod_info::ModInfo;
use workspace_handler::Workspace;

use crate::workspace_handler::SwitchResult;

mod args;
mod branch;
mod mod_info;
mod workspace_handler;

fn main() -> Result<(), Error> {
    let args = args::CliArgs::parse();

    let workspace = Workspace::load_workspace();

    let mut workspace = match workspace {
        Ok(workspace) => workspace,
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => {
                match args.action_context {
                    ActionContext::Init(command) => {
                        let mut current_folder = std::env::current_dir()?;

                        if let Some(folder_name) = command.folderName {
                            current_folder.push(folder_name);
                        }

                        println!(
                            "Initializing workspace in: {},",
                            current_folder.to_str().unwrap()
                        );

                        let mod_info = getModInfoFromUser();
                        let result = Workspace::init(current_folder, mod_info);

                        match result {
                            Ok(workspace) => workspace,
                            Err(e) => {
                                println!("Failed to initialize workspace: {}", e);
                                return Ok(());
                            }
                        };

                        return Ok(());
                    }
                    _ => {
                        println!("You need to initialize a workspace first.\nUse 'ModderCli -h' for help.");
                        return Ok(());
                    }
                };
            }
            _ => {
                println!("Failed to load workspace: {}", err);
                return Ok(());
            }
        },
    };

    handleCommand(&mut workspace, args.action_context)?;

    workspace.save()?;

    Ok(())
}

fn handleCommand(workspace: &mut Workspace, args: ActionContext) -> Result<(), std::io::Error> {
    match args {
        ActionContext::Init(_) => {
            println!("You have already initialized a workspace.");
        }
        ActionContext::Branch(branch) => match branch.action {
            branches::BranchAction::Switch(value) => {
                let res = workspace.switch_branch(&value.branch);

                match res {
                    Ok(e) => match e {
                        SwitchResult::Success => {
                            println!("Switched to branch: {}", value.branch);
                        }
                        SwitchResult::AlreadtInBranch => {
                            println!("Already in branch: {}", value.branch);
                        }
                        SwitchResult::NoFileMove => {
                            println!("Switched to branch: {}", value.branch);
                            println!(
                                "No files were moved to the src folder as the branch is empty."
                            );
                        }
                    },
                    Err(e) => match e.kind() {
                        std::io::ErrorKind::NotFound => {
                            println!("Branch {} not found.", value.branch);
                        }
                        _ => {
                            println!("Failed to switch branch: {}", e);
                        }
                    },
                }
            }
            branches::BranchAction::Create(value) => {
                let branch = branch::Branch::new(value.branch.clone(), "New branch".to_string(), 1);
                let res = workspace.add_branch(branch);

                match res {
                    Ok(_) => {
                        println!("Branch {} created.", value.branch);

                        if value.swap {
                            let switchCommand =
                                branches::BranchAction::Switch(branches::SwitchBranch {
                                    branch: value.branch,
                                });

                            handleCommand(
                                workspace,
                                ActionContext::Branch(branches::BranchComand {
                                    action: switchCommand,
                                }),
                            )?;

                            if value.save {
                                handleCommand(workspace, ActionContext::Save)?;
                            }
                        }
                    }
                    Err(e) => {
                        println!("Failed to create branch: {}", e);
                    }
                }
            }
            branches::BranchAction::Delete(value) => {
                let res = workspace.remove_branch_by_name(&value.value);

                match res {
                    Ok(_) => {
                        println!("Branch {} deleted.", value.value);
                    }
                    Err(e) => {
                        println!("Failed to delete branch: {}", e);
                    }
                }
            }
            branches::BranchAction::List => {
                ListBranches(&workspace)?;
            }
        },
        ActionContext::Save => {
            let res = workspace.save_current_state();

            match res {
                Ok(_) => {
                    println!("Workspace saved.");
                }
                Err(e) => {
                    println!("Failed to save workspace: {}", e);
                }
            }
        }
    }

    Ok(())
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

    return ModInfo::new(name, author, description, Some("main".to_string()));
}

fn ListBranches(workspace: &Workspace) -> Result<(), std::io::Error> {
    // List all branches
    println!("Branches:");

    for branch in &workspace.branches {
        println!(
            "• {} (v.{}): {}",
            branch.name, branch.version, branch.description
        );
    }

    Ok(())
}
