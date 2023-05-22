
use std::{fs, path::{Path, PathBuf}};
use constant::*;
use extractor::{CodeQLExtractor, Extractor};

fn main() {
    let database_path_str = "/home/gdjs2/Desktop/codeql-project/codeql-databases/sample_code/simple_database";
    let ql_ext = CodeQLExtractor::new(database_path_str.to_string());

    ql_ext.extract_funcs();
}
