pub mod codeql;

use std::path::Path;

/**
 Generator traint, which is used to generate analysis tools
 after the response of backend. 
 */
pub trait Generator {
	/**
	 Generating function. 
	 */
	fn gen(&self, p: &Path);
}
