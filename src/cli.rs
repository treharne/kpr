
use clap::{Args, Parser, Subcommand, arg, ValueEnum};

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
    #[command(alias("add"))]
    #[command(alias("kp"))]
    Keep { message: Vec<String> },
    
    /// List your messages
    #[command(alias("ls"))]
    List(ListArgs),

    /// Search your messages
    Search(SearchArgs),

    // re-index the stored messages
    Index,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum DateFormat {
    Ago,
    ISO,
    Epoch,
    EpochMs,
    Human,
}

#[derive(Args)]
pub struct ListArgs {
    // the max number of results to return
    #[arg(short, default_value_t = 10)]
    pub n: usize,

    // the date format to use for timestamps
    #[arg(short, long, value_enum, default_value_t = DateFormat::Ago)]
    #[arg(short, long, value_enum)]
    pub date_format: DateFormat,
}

// struct of SearchArgs
#[derive(Args)]
pub struct SearchArgs {
    pub query: Vec<String>,

    // the max number of results to return
    #[arg(short, default_value_t = 10)]
    pub n: usize,

    // the date format to use for timestamps
    #[arg(short, long, value_enum, default_value_t = DateFormat::Ago)]
    #[arg(short, long, value_enum)]
    pub date_format: DateFormat,
}

pub fn get_cmd() -> Commands {
    Cli::parse().command
}
