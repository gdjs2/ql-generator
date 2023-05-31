use std::{str::FromStr, env};

use ql_generator::{extractor::{CodeQLExtractor, Extractor}, engine::{CodeQLEngine, Engine}};

fn main() {

    env_logger::init();

    let extractor = CodeQLExtractor::new(String::from_str("/home/gdjs2/Desktop/codeql-project/codeql-databases/sample_code/simple_database").unwrap());
    let fns = extractor.extract_funcs();

    for f in &fns {
        println!("{}", f);
    }

    let engine = CodeQLEngine::new(env::var("OPENAI_KEY").unwrap());
    println!("{}", engine.is_alloactor(&fns[0]));

}
