mod constant;
mod engine;
mod extractor;

use std::{fs, path::Path};
use constant::*;

fn main() {

    let allocator = Path::new(PATTERN_PATH).join(ALLOCATOR_FILE);
    let content = fs::read_to_string(&allocator)
                             .expect(format!("Cannot open pattern file {:?}", &allocator).as_str());
    
    println!("{}", content);
}
