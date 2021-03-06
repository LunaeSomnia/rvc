use clap::Parser;

/// An enumerator containing all the possible commands this program can execute with it's parameters
#[derive(Parser, Debug)]
pub enum Command {
    Create {
        #[clap(value_parser)]
        name: String,
        #[clap(value_parser)]
        path: Option<String>,
    },
    Delete,
    State,
    Checkout {
        #[clap(value_parser)]
        branch: String,
    },
    Diff,
    Commit {
        #[clap(value_parser)]
        name: String,
    },
    Push,
    Pull,
    Unlock,
    Revise,
}

/// Argument structure
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}
