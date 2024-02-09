use clap::{Args, Subcommand};

use super::value::Value;

#[derive(Debug, Args)]
pub struct BranchComand {
    #[clap(subcommand)]
    pub action: BranchAction,
}

#[derive(Debug, Args)]
pub struct SwitchBranch {
    pub branch: String,
}

// create branch
#[derive(Debug, Args)]
pub struct CreateBranch {
    pub branch: String,

    /// swap to a branch
    #[arg(short)]
    pub swap: bool,

    /// save the workspace after swapping
    #[arg(short='S')]
    pub save: bool,
}

#[derive(Debug, Subcommand)]
pub enum BranchAction {
    /// Switch to a different branch
    Switch(SwitchBranch),
    Create(CreateBranch),
    Delete(Value),
    List,
}
