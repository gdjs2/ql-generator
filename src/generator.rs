pub mod codeql;

use std::path::Path;

pub trait Generator {
	fn gen(&self, p: &Path);
}
