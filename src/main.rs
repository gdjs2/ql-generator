mod tasks;

use clap::{Args, Parser, Subcommand};
use tasks::{alloc_task, dealloc_task};

#[derive(Parser)]
#[command(author, version, about, long_about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Select all the allocators from the database
    Alloc(AllocArgs),
    /// Select all the deallocators from the database
    Dealloc(DeallocArgs)
}

#[derive(Args)]
pub struct AllocArgs {
    /// The path to the database
    db: String
}

#[derive(Args)]
pub struct DeallocArgs {
    /// The path to the database
    db: String
}

fn main() {
    env_logger::init();

    let _cli = Cli::parse();

    match &_cli.command {
        Commands::Alloc(args) => {
            alloc_task(args);
        }
        Commands::Dealloc(args) => {
            dealloc_task(args);
        }
    }
}
