mod tasks;

use clap::{Args, Parser, Subcommand};
use tasks::{alloc_task, dealloc_task, use_after_free_task};

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
    Dealloc(DeallocArgs),
    /// Detect UAF vulnerabilities
    UAF(UseAfterFreeArgs)
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

#[derive(Args)]
pub struct UseAfterFreeArgs {
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
        Commands::UAF(args) => {
            use_after_free_task(args);
        }
    }
}
