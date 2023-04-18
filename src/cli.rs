
use clap::{Args, Parser, Subcommand, arg};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Store a message for later
    Keep { message: Vec<String> },
    
    /// List your messages
    List(ListArgs),

    /// Search your messages
    Search(SearchArgs),

    // re-index the stored messages
    Index,
}

#[derive(Args)]
pub struct ListArgs {
    // the max number of results to return
    #[arg(short, default_value_t = 10)]
    pub n: usize,
}

// struct of SearchArgs
#[derive(Args)]
pub struct SearchArgs {
    pub query: Vec<String>,

    // the max number of results to return
    #[arg(short, default_value_t = 10)]
    pub n: usize,
}

pub fn get_cmd() -> Commands {
    Cli::parse().command
}
