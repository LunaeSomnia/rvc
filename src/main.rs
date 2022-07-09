use std::{env::current_dir, path::Path};

use crate::{
    cli::{Args, Command},
    rvc::commit::Commit,
};
use clap::Parser;
use colored::Colorize;
use rvc::repository::Repository;

pub mod cli;
pub mod rvc;
pub mod utils;

fn get_current_repository() -> Result<Repository, String> {
    let repo = current_dir().unwrap();
    let supposed_rvc_path = repo.join(".rvc");
    if supposed_rvc_path.exists() {
        Repository::retreive(repo)
    } else {
        Err(String::from("The path does not exist"))
    }
}

fn print_error(error_msg: &str) {
    println!(
        "{} {}",
        format!("{}", "error:".bold().bright_red()),
        error_msg
    )
}

fn main() -> Result<(), String> {
    let args: Args = Args::parse();

    println!("");

    match &args.command {
        Command::Create { name, path } => {
            let repo_path = match path {
                Some(p) => Path::new(p).canonicalize().unwrap(),
                _ => current_dir().unwrap(),
            };

            match Repository::create(name, repo_path) {
                Ok(mut repo) => {
                    repo.checkout_lastest();
                    repo.save();
                    println!(
                        "The repository {} was created successfully",
                        format!("{}", repo.name.bright_green())
                    );
                }
                Err(e) => {
                    print_error(&e);
                }
            }
        }
        Command::Delete => {
            if let Ok(repo) = get_current_repository() {
                repo.delete();
            } else {
                print_error("No repository found in the working directory")
            }
        }
        Command::State => {
            if let Ok(repo) = get_current_repository() {
                println!("{}", &repo);
            } else {
                print_error("No repository found in the working directory")
            }
        }
        Command::Commit { name } => {
            let mut repo = get_current_repository().unwrap();
            match Commit::create(name, &mut repo) {
                Ok(commit) => {
                    println!("{}", commit);
                    repo.push_commit(commit);
                    repo.checkout_lastest();
                    repo.save()
                }
                Err(e) => print_error(&e),
            }
        }
        Command::Checkout { branch } => {
            println!(
                "COMMAND: Checkout a new branch in the current repository with name '{}'",
                branch
            );
        }
        _ => todo!(),
    }

    Ok(())
}
