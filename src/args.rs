use clap::{Parser, Subcommand};

pub mod branches;

use branches::BranchComand;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// A simple modding tool to manage mod files so I don't go insane working on them.
pub struct CliArgs {
    #[command(subcommand)]
    pub action_context: ActionContext,
}

#[derive(Debug, Subcommand)]
pub enum ActionContext {

    /// Initialize a new workspace in the current directory
    Init,

    /// Manage the branch of the mod that is loaded in the work space, allows for switching between branches
    Branch(BranchComand),
}
