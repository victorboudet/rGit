use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Init,
    Add {
        files: Vec<String>,
    },
    Commit {
        #[arg(short, long)]
        message: String,
    },
    Status,
    Log,
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
