#![allow(non_snake_case)]
use clap::Parser;

mod args;

fn main() {
    let args = args::CliArgs::parse();
    
    println!("{:?}", args);

    match args.action_context {
        args::ActionContext::Init => {
            println!("Init");
        }
        args::ActionContext::Branch(branch) => {
            match branch.action {
                args::branches::BranchAction::Switch(value) => {
                    println!("Switch to branch: {}", value.value);
                }
                args::branches::BranchAction::Create(value) => {
                    println!("Create branch: {}", value.value);
                }
                args::branches::BranchAction::Delete(value) => {
                    println!("Delete branch: {}", value.value);
                }
                args::branches::BranchAction::List => {
                    println!("List branches");
                }
            }
        }
    }
}
