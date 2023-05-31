use core::fmt;
use std::{path::{PathBuf, Path}, process::{Command, Output}, fs::{self, create_dir}, fmt::Debug, str::FromStr};

use serde::{Deserialize, Serialize};
use serde_json::{Value, Number};
use url::Url;

use crate::constant::{self, WORK_DIR, SELECT_FUN_RESULT_JSON};

/**
This is front end of the whole engine, which is reponsible
for extracting all the functions.
 */
pub trait Extractor {
	/**
	 * Extract all the functions.
	 */
	fn extract_funcs(&self) -> Vec<Func>;
}

/**
 This is an implementation of Extractor, which uses CodeQL.
 The input should be a valid CodeQL database.
 */
pub struct CodeQLExtractor {
	database_pathbuf: PathBuf
}

/**
 Function query result for CodeQL

 * block: [`Option`]<[`String`]> is for the function block got
 from source code
 */
#[derive(Debug)]
pub struct Func {
	pub ret_type: String,
	pub name: String,
	pub parameters: String,
	pub url: BlockUrl,
	pub block: Option<String>
}

impl fmt::Display for Func {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {} ({}) {{\n{}}}\n", self.ret_type, self.name, self.parameters, self.block.clone().unwrap_or(String::from_str("/*Empty*/").unwrap()))
	}
}

/**
 This is the url segment in the CodeQL query result, recording
 the position of the functions.

* uri: File URI  
* start_line: The start line of the function block
* start_column: The start column of the function block
* end_line: The end line of the function block
* end_column: The end column of the function block
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct BlockUrl {
	uri: String,
	#[serde(alias = "startLine")]
	start_line: Number,
	#[serde(alias = "startColumn")]
	start_column: Number,
	#[serde(alias = "endLine")]
	end_line: Number,
	#[serde(alias = "endColumn")]
	end_column: Number
}

/**
 This is the implementation for CodeQL Extractor, which includes
 several key functions for CodeQL functionality.
 */
impl CodeQLExtractor {

	/**
	 Create a new CodeQL Extractor.

	 * database_path_str: a String specify the path of the CodeQL database

	 */
	pub fn new(database_path_str: String) -> Self {
		let database_pathbuf = PathBuf::new().join(database_path_str);
		Self { database_pathbuf }
	}

	/**
	 Execute a ql file

	 * ql: The path to the to-be-executed ql
	 * res: The path to the result bqrs file

	 * return: [`Result`]<[`Output`], [`std::io::Error`]>, which is a result
	 wrapper of Output and Error. This is the execution result of the CodeQL
	 command.

	 */
	fn exec_ql(&self, ql: &Path, res: &Path) -> Result<Output, std::io::Error>{

		let mut cmd = Command::new(constant::CODEQL_BIN);
			
		let _opt = cmd.args(["query", "run"])
			.arg(ql.to_str().expect(format!("The path of ql {:?} is not valid", ql).as_str()))
			.args(["-o", res.to_str().expect(format!("The path of result {:?} is not valid", res).as_str())])
			.args(["-d", self.database_pathbuf.to_str().expect(format!("The database path {:?} is not valid", self.database_pathbuf).as_str())])
			.output();

		log::debug!("Execute CodeQL Command: {:?}", cmd);
		return _opt;

	}

	/**
	 Execute the ql file for selecting all the functions.

	 * return: A [`PathBuf`] to the result bqrs file
	 */
	fn exec_select_ql(&self) -> PathBuf {

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

		let opt = self.exec_ql(select_func_ql.as_path(), select_result.as_path());
		if let Ok(o) = opt {
			log::debug!("Execute ql file stdout: {}", String::from_utf8(o.stdout).unwrap());
			log::debug!("Execute ql file stderr: {}", String::from_utf8(o.stderr).unwrap());
		} else {
			opt.unwrap();
		}

		return select_result;

	}

	/**
	 Convert bqrs result to json result.

	 * bqrs_path: [`Path`] to bqrs result file
	 * json_path: [`Path`] to to-stored json file

	 */
	fn convert_bqrs2json(bqrs_path: &Path, json_path: &Path) {

		let mut cmd = Command::new(constant::CODEQL_BIN);

		let _opt = cmd.args(["bqrs", "decode"])
			.arg(bqrs_path.to_str().expect("The path of result database is not valid"))
			.args(["-o", json_path.to_str().expect("The path of result json is not valid")])
			.args(["--format=json", "--entities=url"])
			.output();

		log::debug!("Execute CodeQL Command: {:?}", cmd);

	}

	/**
	 Parse the json result file to [`Vec`] of [`Func`]

	 * return: [`Vec`]<[`Func`]>, which is metadata for all the functions
	 */
	fn parse_result_json(json_path: &Path) -> Vec<Func> {

		let mut ret = Vec::new();

		let json_txt = fs::read_to_string(json_path).unwrap();
		log::debug!("Read {:?} file: {}", json_path, json_txt);

		let parsed_json: Value = serde_json::from_str(&json_txt).unwrap();
		let tuples_value = &parsed_json["#select"]["tuples"];

		for tuple in tuples_value.as_array().unwrap() {
			let url = serde_json::from_value(tuple[3]["url"].clone()).unwrap();
			ret.push(Func {
				ret_type: serde_json::from_value(tuple[0].clone()).unwrap(),
				name: serde_json::from_value(tuple[1].clone()).unwrap(),
				parameters: serde_json::from_value(tuple[2].clone()).unwrap(),
				block: Self::get_fn_block(&url),
				url
			});
		}

		log::debug!("parsed json: {:?}", ret);

		return ret;

	}

	/**
	 Get single function including declaration and body by
	 a matadata.

	 * f: [`Func`], function metadata got from CodeQL query result
	 * return: [`Option`]<[`String`]>, the function body
	 	* [`None`]: if the source file cannot find, i.e., "/"
	 */
	fn get_fn_block(url: &BlockUrl) -> Option<String> {

		let parsed_url = Url::parse(&url.uri).unwrap();
		log::debug!("get_fn: {}, {}", parsed_url.scheme(), parsed_url.path());

		if parsed_url.path() == "/" {
			return None
		}

		let file_txt = fs::read_to_string(parsed_url.path()).unwrap();
		let file_txt_dup = file_txt.clone();

		let split = file_txt_dup.split("\n");
		let mut start_idx = 0;
		let mut end_index = 0;
		for l in 0..url.start_line.as_u64().unwrap()-1 {
			start_idx += split.clone().nth(usize::try_from(l).unwrap()).unwrap().len()+1;
		}
		start_idx += usize::try_from(url.start_column.as_u64().unwrap()).unwrap();

		for l in 0..url.end_line.as_u64().unwrap()-1 {
			end_index += split.clone().nth(usize::try_from(l).unwrap()).unwrap().len()+1;
		}
		end_index += usize::try_from(url.end_column.as_u64().unwrap()).unwrap();

		log::debug!("start_idx: {}, end_inx: {}", start_idx, end_index);
		log::debug!("Get function: {}", &file_txt[start_idx..end_index-1]);

		Some(String::from_str(&file_txt[start_idx..end_index-1]).unwrap())

	}

}

/**
 Extractor implementation for CodeQL Extractor
 */
impl Extractor for CodeQLExtractor {
	/**
	 Extract all the functions from a CodeQL database
	 
	 return: [`Vec`]<[`String`]>, all the functions extracted from
	 a CodeQL database
	 */
	fn extract_funcs(&self) -> Vec<Func> {

		let bqrs_path = self.exec_select_ql();
		let json_path = Path::new(WORK_DIR).join(SELECT_FUN_RESULT_JSON);
		CodeQLExtractor::convert_bqrs2json(bqrs_path.as_path(), json_path.as_path());
	
		let parsed = CodeQLExtractor::parse_result_json(json_path.as_path());

		// let ret = CodeQLExtractor::get_funcs(&parsed);
		log::debug!("Extracted functions: {:?}", parsed);

		return parsed;
	}
}
