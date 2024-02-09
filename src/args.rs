use clap::{Args, Parser, Subcommand};

pub mod value;
pub mod branches;

use branches::BranchComand;

#[derive(Parser, Debug)]
#[command()]
/// A simple modding tool to manage mod files so I don't go insane working on them.
pub struct CliArgs {
    #[command(subcommand)]
    pub action_context: ActionContext,
}

#[derive(Args, Debug)]
pub struct InitCommand {
    pub folderName: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum ActionContext {

    /// Initialize a new workspace in the current directory
    Init(InitCommand),

    /// Manage the branch of the mod that is loaded in the work space, allows for switching between branches
    Branch(BranchComand),

    /// Save the current state of the mod to the current branch
    Save,
}
