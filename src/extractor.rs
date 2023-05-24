use std::{path::{PathBuf, Path}, process::Command, fs::{self, create_dir}};

use crate::constant::{self, WORK_DIR};

pub trait Extractor {
	fn extract_funcs(&self) -> Vec<String> {
		vec![String::from("int main() {}")]
	}
}

pub struct CodeQLExtractor {
	database_pathbuf: PathBuf
}

impl CodeQLExtractor {
	pub fn new(database_path_str: String) -> Self {
		let database_pathbuf = PathBuf::new().join(database_path_str);
		Self { database_pathbuf }
	}

	pub fn exec_ql(&self, ql: &Path, res: &Path) {
		let mut cmd = Command::new(constant::CODEQL_BIN);
			
		let _opt = cmd.args(["query", "run"])
			.args([ql.to_str().expect(format!("The path of ql {:?} is not valid", ql).as_str())])
			.args(["-o", res.to_str().expect(format!("The path of result {:?} is not valid", res).as_str())])
			.args(["-d", self.database_pathbuf.to_str().expect(format!("The database path {:?} is not valid", self.database_pathbuf).as_str())])
			.output();
	}

	pub fn exec_select_ql(&self) {
		let select_func_ql = 
			PathBuf::new()
					.join(constant::QLS_PATH)
					.join(constant::SELECT_FUNC_QL_FILE);
		
		let work_dir = Path::new(WORK_DIR);
		if !work_dir.is_dir() {
			create_dir(work_dir).expect("Create work dir failed");
		}

		let select_result = 
			PathBuf::new()
					.join(constant::WORK_DIR)
					.join(constant::SELECT_FUN_RESULT_BQRS);

		let _output = 
			Command::new("codeql")
			.args([
				"query", 
				"run", 
				select_func_ql.to_str().expect("The select ql path string is not valid"), 
				"-o", 
				select_result.to_str().expect("The select store path string is not valid"),
				"-d",
				self.database_pathbuf.to_str().expect("The database path is not a valid string")])
			.output();
	}

	pub fn convert_bqrs2json(bqrs_path: &Path, json_path: &Path) {

	}
}

impl Extractor for CodeQLExtractor {
	fn extract_funcs(&self) -> Vec<String> {
		let mut ret = Vec::new();

		self.exec_select_ql();

		ret.push("int main() {}".to_string());
		return ret;

	}
}
