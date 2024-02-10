use clap::{Args, Parser, Subcommand};

use branches::BranchComand;

pub mod value;
pub mod branches;   

#[derive(Parser, Debug)]
#[command()]
/// A simple modding tool to manage mod files so I don't go insane working on them.
pub struct CliArgs {
    #[command(subcommand)]
    pub action_context: ActionContext,

    #[arg(short='F', long)]
    pub force: bool,
}

#[derive(Args, Debug)]
pub struct InitCommand {
    pub folder_name: Option<String>,
}

#[derive(Debug, Args)]
pub struct RestoreBranch {
    pub version: Option<i32>,

    #[arg(short, long)]
    pub file: Option<String>,

}

#[derive(Debug, Subcommand)]
pub enum ActionContext {
    /// Initialize a new workspace in the current directory
    Init(InitCommand),

    /// Manage the branch of the mod that is loaded in the work space, allows for switching between branches
    Branch(BranchComand),

    /// Save the current state of the mod to the current branch
    Save,

    Restore(RestoreBranch),
}
