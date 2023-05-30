use std::{fs, path::{Path, PathBuf}, str::FromStr};

use ql_generator::{extractor::{CodeQLExtractor, Extractor}, constant::{WORK_DIR, SELECT_FUN_RESULT_JSON}};

fn main() {

    env_logger::init();

    let extractor = CodeQLExtractor::new(String::from_str("/home/gdjs2/Desktop/codeql-project/codeql-databases/sample_code/simple_database").unwrap());
    let fns = extractor.extract_funcs();

    for f in fns {
        println!("{}", f);
    }

}
