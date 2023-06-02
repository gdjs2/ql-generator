use std::path::Path;

use ql_generator::{extractor::{CodeQLExtractor, Extractor}, engine::{ChatGPTEngine, Engine}, generator::{CodeQLGenerator, Generator}, constant};

use crate::AllocArgs;

pub fn alloc_task(args: &AllocArgs) {
	log::info!(
		"[Command Alloc] Creating CodeQL Extractor using database {}",
		&args.db
	);
	let extractor = CodeQLExtractor::new(args.db.clone());

	log::info!("[Command Alloc] Extracting functions...");
	let funcs = extractor.extract_funcs();
	log::info!(
		"[Command Alloc] Extracted {} functions in total",
		funcs.len()
	);

	log::info!("[Command Alloc] Creating ChatGPT Engine...");
	let engine = ChatGPTEngine::new(std::env::var("OPENAI_KEY").unwrap());
	let mut left_f = Vec::new();

	log::info!("[Command Alloc] Start asking...");
	for f in &funcs {
		let res = engine.is_allocator(f);
		if res {
			left_f.push(f);
		}
		log::info!(
			"[Command Alloc] Function{{ {} }}, Result{{ {} }}",
			f.name,
			res
		);
	}
	log::info!(
		"[Command Alloc] End asking, {} functions left",
		left_f.len()
	);

	log::info!("[Command Alloc] Generating ql code...");
	let mut ql_code = String::new();
	for f in left_f {
		ql_code.push_str(&format!("\t\tor fun.hasGlobalName(\"{}\")\n", f.name));
	}
	log::info!("[Command Alloc] Generated ql:\n{}", ql_code);

	log::info!("[Command Alloc] Creating CodeQL Generator...");
	let gen = CodeQLGenerator::new(
		Path::new(constant::QLS_PATH)
			.join(constant::ALLOCATOR_DIR)
			.to_str()
			.unwrap(),
		vec![constant::ALLOCATOR_FILE],
	);

	log::info!("[Command Alloc] Generating...");
	gen.gen(Path::new("./tmp"), &vec![ql_code]);
	log::info!("[Command Alloc] End generating");
}
