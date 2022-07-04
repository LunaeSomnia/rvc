use clap::Parser;

#[derive(Parser, Debug)]
pub enum Command {
    Create {
        #[clap(value_parser)]
        name: String,
        #[clap(value_parser)]
        path: Option<String>,
    },
    Delete {
        #[clap(value_parser)]
        name: String,
    },
    Checkout {
        #[clap(value_parser)]
        branch: String,
    },

    Commit,
    Push,
    Pull,
    Unlock,
    Revise,
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[clap(subcommand)]
    pub command: Command,
}
