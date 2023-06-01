use std::{str::FromStr, env};

use ql_generator::{extractor::{CodeQLExtractor, Extractor}, engine::{ChatGPTEngine, Engine}};

#[test]
fn test_codeql_extractor_and_engine() {

	env_logger::init();

	let extractor = CodeQLExtractor::new(String::from_str("/Users/gdjs2/Desktop/codeql-project/codeql-databases/user_defined_alloc/user_defined_alloc_database").unwrap());
	let fns = extractor.extract_funcs();

	let engine = ChatGPTEngine::new(env::var("OPENAI_KEY").unwrap());

	for f in &fns {
		println!("{}", f);
		log::debug!("[test_codeql_extractor_and_engine]: {}", engine.is_allocator(&f));
	}

}