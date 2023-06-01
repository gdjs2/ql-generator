use std::{str::FromStr, env, path::Path};

use ql_generator::{extractor::{CodeQLExtractor, Extractor}, engine::{ChatGPTEngine, Engine}, generator::{self, CodeQLGenerator, Generator}};

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

#[test]
fn test_codeql_generator() {

	env_logger::init();

	let pts_s = vec!["./allocator.ql"];
	let generator = CodeQLGenerator::new("/home/gdjs2/Desktop/codeql-project/ql-generator/ql-generator-qls", pts_s);

	generator.gen(Path::new("./tmp/.work_dir"));

}