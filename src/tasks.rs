use std::path::Path;

use ql_generator::{
    constant,
    engine::{chatgpt::ChatGPTEngine, Engine},
    extractor::{codeql::CodeQLExtractor, Extractor},
    generator::{codeql::CodeQLGenerator, Generator},
};

use crate::{AllocArgs, DeallocArgs};

/**
 The task for generating ql files for allocator selecting.

 * args: &[`AllocArgs`], which is the arguments for allocator task
 */
pub fn alloc_task(args: &AllocArgs) {

    // Create CodeQL Extractor
    log::info!(
        "[Command Alloc] Creating CodeQL Extractor using database {}",
        &args.db
    );
    let extractor = CodeQLExtractor::new(args.db.clone());

    // Extract Functions
    log::info!("[Command Alloc] Extracting functions...");
    let funcs = extractor.extract_funcs();
    log::info!(
        "[Command Alloc] Extracted {} functions in total",
        funcs.len()
    );

    // Create ChatGPT Engine
    log::info!("[Command Alloc] Creating ChatGPT Engine...");
    let engine = ChatGPTEngine::new(std::env::var("OPENAI_KEY").unwrap());

    // Define the vector for the left functions after asking for Engine
    let mut left_f = Vec::new();

    // Ask for Engine
    log::info!("[Command Alloc] Start asking...");
    for f in &funcs {
        let res = engine.is_allocator(f);
        if res {
            left_f.push(f);
        }
        log::info!(
            "[Command Alloc] Function{{ {} }}, Result{{ {} }}",
            f.name,
            res
        );
    }
    log::info!(
        "[Command Alloc] End asking, {} functions left",
        left_f.len()
    );

    // Create the CodeQL Generator
    log::info!("[Command Alloc] Creating CodeQL Generator...");
    let gen = CodeQLGenerator::new(
        Path::new(constant::QLS_PATH)
            .join(constant::ALLOCATOR_DIR)
            .to_str()
            .unwrap(),
        vec![constant::ALLOCATOR_FILE],
        left_f,
    );

    log::info!("[Command Alloc] Generating...");
    gen.gen(Path::new("./tmp"));
    log::info!("[Command Alloc] End generating");
}

pub fn dealloc_task(args: &DeallocArgs) {
    log::info!(
        "[Command Dealloc] Creating CodeQL Extractor using database {}",
        &args.db
    );
    let extractor = CodeQLExtractor::new(args.db.clone());

    log::info!("[Command Dealloc] Extracting functions...");
    let funcs = extractor.extract_funcs();
    log::info!(
        "[Command Dealloc] Extracted {} functions in total",
        funcs.len()
    );

    log::info!("[Command Dealloc] Creating ChatGPT Engine...");
    let engine = ChatGPTEngine::new(std::env::var("OPENAI_KEY").unwrap());
    let mut left_f = Vec::new();

    log::info!("[Command Dealloc] Start asking...");
    for f in &funcs {
        let res = engine.is_deallocator(f);
        if res {
            left_f.push(f);
        }
        log::info!(
            "[Command Dealloc] Function{{ {} }}, Result{{ {} }}",
            f.name,
            res
        );
    }
    log::info!(
        "[Command Dealloc] End asking, {} functions left",
        left_f.len()
    );

    log::info!("[Command Dealloc] Creating CodeQL Generator...");
    let gen = CodeQLGenerator::new(
        Path::new(constant::QLS_PATH)
            .join(constant::DEALLOCATOR_DIR)
            .to_str()
            .unwrap(),
        vec![constant::DEALLOCATOR_FILE],
        left_f,
    );

    log::info!("[Command Dealloc] Generating...");
    gen.gen(Path::new("./tmp"));
    log::info!("[Command Dealloc] End generating");
}
