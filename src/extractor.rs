pub mod codeql;

use core::fmt;
use std::{
    fmt::Debug,
    str::FromStr,
};

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
Structure for functions

* block: [`Option`]<[`String`]> is for the function block got
from source code
*/
#[derive(Debug)]
pub struct Func {
    pub ret_type: String,
    pub name: String,
    pub parameters: String,
    pub block: Option<String>,
}

impl fmt::Display for Func {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} ({}) {{\n{}}}\n",
            self.ret_type,
            self.name,
            self.parameters,
            self.block
                .clone()
                .unwrap_or(String::from_str("/*Empty*/").unwrap())
        )
    }
}
