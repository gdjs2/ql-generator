use std::path::PathBuf;

pub trait Generator {
	fn gen() -> PathBuf;
}

pub struct CodeQLGenerator {
	ql: PathBuf
}

