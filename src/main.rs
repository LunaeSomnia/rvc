use crate::cli::{Args, Command};
use clap::Parser;
use rcv::Rcv;

pub mod cli;
pub mod dirs;
pub mod rcv;

fn main() -> Result<(), String> {
    let args: Args = Args::parse();

    let mut rcv_status = Rcv::retreive();

    eprintln!("{:?}", rcv_status);

    match &args.command {
        // Creates a new repository in the current working directory
        Command::Create { name, path } => {
            rcv_status.create_repository(name, path);
        }
        Command::Delete { name } => {
            rcv_status.delete_repository(name);
        }
        Command::State => {
            println!("{}", rcv_status)
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
        rcv_status.changed_state = false;
        rcv_status.save();
    }

    Ok(())
}
