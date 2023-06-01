use clap::{Parser, Args, Subcommand};
use ql_generator::{extractor::{CodeQLExtractor, Extractor}, engine::{ChatGPTEngine, Engine}};

#[derive(Parser)]
#[command(author, version, about, long_about)]
struct Cli {
	#[command(subcommand)]
	command: Commands
}

#[derive(Subcommand)]
enum Commands {
	/// Select all the allocators from the database
	Alloc(AllocArgs)
}

#[derive(Args)]
struct AllocArgs {
	/// The path to the database
	db: String
}



fn main() {

	env_logger::init();

	let _cli = Cli::parse();

	match &_cli.command {
		Commands::Alloc(args) => {
			
			log::info!("[Command Alloc] Creating CodeQL Extractor using database {}", &args.db);
			let extractor = CodeQLExtractor::new(args.db.clone());

			log::info!("[Command Alloc] Extracting functions...");
			let funcs = extractor.extract_funcs();
			log::info!("[Command Alloc] Extracted {} functions in total", funcs.len());

			log::info!("[Command Alloc] Creating ChatGPT Engine...");
			let engine = ChatGPTEngine::new(std::env::var("OPENAI_KEY").unwrap());
			
			log::info!("[Command Alloc] Start asking...");
			for f in &funcs {
				let res = engine.is_allocator(f);
				log::info!("[Command Alloc] Function{{ {} }}, Result{{ {} }}", f.name, res);
			}

			log::info!("[Command Alloc] End asking");
		}
	}
}
