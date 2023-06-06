use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use fs_extra::dir::{copy, CopyOptions};

use crate::constant;

use super::Generator;

pub struct Pts {
    pub f: PathBuf,
    pub s: String
}

/**
The CodeQL Generator, which will generate a CodeQL pack
for analyzing vulnerabilities.
*/
pub struct CodeQLGenerator {
    // The directory to store the CoedQL pack
    ql_dir: PathBuf,
    // The file to be replaced for [idx]th replace point
    v: Vec<Pts>,
}

/**
A implementation of generator based on CodeQL.
*/
impl CodeQLGenerator {
    /**
    Create a new CodeQL Generator.

    * ql_dir_s: &[`str`] The string of the path to the original codeql
    pack directory
    * replace_pts_s: [`Vec`]<&[`str`]> A vector stores all the files' paths
    to be replaced by the genereator
    * funcs: [`Vec`]<&[`Func`]> A vector stores all the
    */
    pub fn new(ql_dir_s: &str, v: Vec<Pts>) -> Self {
        let ql_dir = PathBuf::new().join(ql_dir_s);
        CodeQLGenerator { ql_dir, v }
    }
}

impl Generator for CodeQLGenerator {
    fn gen(&self, p: &Path) {
        log::debug!(
            "[CodeQL Generator][gen()] Initializing pack at {{ {} }}",
            p.to_str().unwrap()
        );

        let opt = Command::new(constant::CODEQL_BIN)
            .args([
                "pack",
                "init",
                "--dir=.",
                format!("tmp/{}", self.ql_dir.file_name().unwrap().to_str().unwrap()).as_str(),
            ])
            .current_dir(&p)
            .output();

        if let Ok(o) = opt {
            log::debug!(
                "Execute codeql init stdout: {}",
                String::from_utf8(o.stdout).unwrap()
            );
            log::debug!(
                "Execute codeql init stderr: {}",
                String::from_utf8(o.stderr).unwrap()
            );
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
        for p in &self.v {
            let tar_file = PathBuf::new().join(&tar_dir).join(&p.f);
            log::debug!(
                "[CodeQL Generator][gen()] Generating {{ {} }}<{}>",
                tar_file.to_str().unwrap(),
                idx
            );
            let file = fs::read_to_string(&tar_file).unwrap();
            let mut pts = constant::PATTERN_KEY.replace("{}", idx.to_string().as_str());
            log::debug!("[CodeQL Generator][gen()] Pattern Key is {{ {} }}", pts);
            log::debug!(
                "[CodeQL Generator][gen()] Replace Pattern Key to \"{}\"",
                self.v[idx].s
            );
            pts = file.replace(&pts, &self.v[idx].s);
            fs::write(tar_file.to_str().unwrap(), pts).unwrap();

            idx += 1;
        }

        let opt = Command::new(constant::CODEQL_BIN)
            .args(["pack", "add", "codeql/cpp-all"])
            .current_dir(&tar_dir)
            .output();

        if let Ok(o) = opt {
            log::debug!(
                "Execute codeql add cpp-all stdout: {}",
                String::from_utf8(o.stdout).unwrap()
            );
            log::debug!(
                "Execute codeql add cpp-all stderr: {}",
                String::from_utf8(o.stderr).unwrap()
            );
        } else {
            opt.unwrap();
        }

        let opt = Command::new(constant::CODEQL_BIN)
            .args(["pack", "install"])
            .current_dir(&tar_dir)
            .output();

        if let Ok(o) = opt {
            log::debug!(
                "Execute codeql pack install stdout: {}",
                String::from_utf8(o.stdout).unwrap()
            );
            log::debug!(
                "Execute codeql pack install stderr: {}",
                String::from_utf8(o.stderr).unwrap()
            );
        } else {
            opt.unwrap();
        }
    }
}
