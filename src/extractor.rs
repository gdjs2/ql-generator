use std::{path::Path, process::Command};

use crate::constant;

pub trait Extractor {
	fn extract_funcs(&self, _database: &Path) -> Vec<String> {
		vec![String::from("int main() {}")]
	}
}

pub struct CodeQLExtractor {
	
}

impl Extractor for CodeQLExtractor {
	fn extract_funcs(&self, _database: &Path) -> Vec<String> {
		let mut ret = Vec::new();
		// let output = Command::new("codeql")
		// 								   .args(["query", "run", constant::SELECT_FUNC_QL_FILE, "-o", ""])

	}
}
