use std::{str::FromStr, env};

use ql_generator::{extractor::{CodeQLExtractor, Extractor}, engine::{ChatGPTEngine, Engine}};

fn main() {

    env_logger::init();

    let extractor = CodeQLExtractor::new(String::from_str("/Users/gdjs2/Desktop/codeql-databases/sample_code/simple_1_database").unwrap());
    let fns = extractor.extract_funcs();

    for f in &fns {
        println!("{}", f);
    }

    let engine = ChatGPTEngine::new(env::var("OPENAI_KEY").unwrap());
    println!("{}", engine.is_allocator(&fns[0]));

}
