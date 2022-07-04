use crate::cli::{Args, Command};
use clap::Parser;
use dirs::Dirs;
use rcv::Rcv;

pub mod cli;
pub mod dirs;
pub mod rcv;

fn main() -> Result<(), String> {
    let args: Args = Args::parse();

    let dirs = Dirs::init().expect("No default directories found!");
    let mut rcv_status = Rcv::retreive(&dirs);

    match &args.command {
        // Creates a new repository in the current working directory
        Command::Create { name } => {
            rcv_status.create_repository(name, &dirs);
        }
        Command::Delete { name } => {
            rcv_status.delete_repository(name);
        }
        Command::Checkout { branch } => {
            println!(
                "COMMAND: Checkout a new branch in the current repository with name '{}'",
                branch
            );
        }
        Command::Commit => {}
        Command::Pull => {}
        Command::Push => {}
        Command::Revise => {}
        Command::Unlock => {}
    }

    if rcv_status.changed_state {
        rcv_status.save(&dirs);
    }

    Ok(())
}
