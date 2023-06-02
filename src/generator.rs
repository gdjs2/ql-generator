use std::{
	fs,
	path::{Path, PathBuf},
	process::Command,
};

use fs_extra::dir::{copy, CopyOptions};

use crate::constant;

pub trait Generator {
	fn gen(&self, p: &Path, v: &Vec<String>);
}

pub struct CodeQLGenerator {
	ql_dir: PathBuf,
	replace_pts: Vec<PathBuf>,
}

impl CodeQLGenerator {
	pub fn new(ql_dir_s: &str, replace_pts_s: Vec<&str>) -> Self {
		let ql_dir = PathBuf::new().join(ql_dir_s);
		let mut replace_pts = Vec::new();
		for s in replace_pts_s {
			replace_pts.push(PathBuf::new().join(s));
		}
		CodeQLGenerator {
			ql_dir,
			replace_pts,
		}
	}
}

impl Generator for CodeQLGenerator {
	fn gen(&self, p: &Path, v: &Vec<String>) {

		log::debug!(
			"[CodeQL Generator][gen()] Initializing pack at {{{}}}",
			p.to_str().unwrap()
		);

		let opt = Command::new(constant::CODEQL_BIN)
			.args(["pack", "init", "--dir=.", "gdjs2/alloc"])
			.current_dir(&p)
			.output();

		if let Ok(o) = opt {
			log::debug!("Execute codeql init stdout: {}", String::from_utf8(o.stdout).unwrap());
			log::debug!("Execute codeql init stderr: {}", String::from_utf8(o.stderr).unwrap());
		} else {
			opt.unwrap();
		}

		log::debug!(
			"[CodeQL Generator][gen()] Copy QL directory from {{{:?}}} to {{{:?}}}",
			self.ql_dir,
			p
		);
		let ops = CopyOptions::new().overwrite(true);
		let res = copy(&self.ql_dir, p, &ops);
		log::debug!("[CodeQL Generator][gen()] Copy Result {{{:?}}}", res);

		let tar_dir = PathBuf::new()
			.join(p)
			.join(self.ql_dir.file_name().unwrap());
		log::debug!(
			"[CodeQL Generator][gen()] The target directory is \"{:?}\"",
			tar_dir
		);

		log::debug!("[CodeQL Generator][gen()] Start generating qls...");
		let mut idx = 0;
		for pt in &self.replace_pts {
			let tar_file = PathBuf::new().join(&tar_dir).join(pt);
			log::debug!(
				"[CodeQL Generator][gen()] Generating {{{}}}<{}>",
				tar_file.to_str().unwrap(),
				idx
			);
			let file = fs::read_to_string(&tar_file).unwrap();
			let mut pts = constant::PATTERN_KEY.replace("{}", idx.to_string().as_str());
			log::debug!("[CodeQL Generator][gen()] Pattern Key is {{{}}}", pts);
			log::debug!(
				"[CodeQL Generator][gen()] Replace Pattern Key to \"{}\"",
				v[idx]
			);
			pts = file.replace(&pts, &v[idx]);
			fs::write(tar_file.to_str().unwrap(), pts).unwrap();

			idx += 1;
		}

		let opt = Command::new(constant::CODEQL_BIN)
			.args(["pack", "add", "codeql/cpp-all"])
			.current_dir(&tar_dir)
			.output();

		if let Ok(o) = opt {
			log::debug!("Execute codeql add cpp-all stdout: {}", String::from_utf8(o.stdout).unwrap());
			log::debug!("Execute codeql add cpp-all stderr: {}", String::from_utf8(o.stderr).unwrap());
		} else {
			opt.unwrap();
		}

		let opt = Command::new(constant::CODEQL_BIN)
			.args(["pack", "install"])
			.current_dir(&tar_dir)
			.output();

		if let Ok(o) = opt {
			log::debug!("Execute codeql pack install stdout: {}", String::from_utf8(o.stdout).unwrap());
			log::debug!("Execute codeql pack install stderr: {}", String::from_utf8(o.stderr).unwrap());
		} else {
			opt.unwrap();
		}
		
	}
}
