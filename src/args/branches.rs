use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct BranchComand {
    #[clap(subcommand)]
    pub action: BranchAction,
}


#[derive(Debug, Subcommand)]
pub enum BranchAction {
    /// Switch to a different branch
    Switch(Value),
    Create(Value),
    Delete(Value),
    List,
}

#[derive(Debug, Args)]
pub struct Value {
    pub value: String,
}
