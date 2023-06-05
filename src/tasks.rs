use std::path::Path;

use ql_generator::{
    constant,
    extractor::{Extractor, codeql::CodeQLExtractor},
    generator::{codeql::CodeQLGenerator, Generator}, engine::{chatgpt::ChatGPTEngine, Engine},
};

use crate::{AllocArgs, DeallocArgs};

pub fn alloc_task(args: &AllocArgs) {
    log::info!(
        "[Command Alloc] Creating CodeQL Extractor using database {}",
        &args.db
    );
    let extractor = CodeQLExtractor::new(args.db.clone());

    log::info!("[Command Alloc] Extracting functions...");
    let funcs = extractor.extract_funcs();
    log::info!(
        "[Command Alloc] Extracted {} functions in total",
        funcs.len()
    );

    log::info!("[Command Alloc] Creating ChatGPT Engine...");
    let engine = ChatGPTEngine::new(std::env::var("OPENAI_KEY").unwrap());
    let mut left_f = Vec::new();

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

    log::info!("[Command Alloc] Creating CodeQL Generator...");
    let gen = CodeQLGenerator::new(
        Path::new(constant::QLS_PATH)
            .join(constant::ALLOCATOR_DIR)
            .to_str()
            .unwrap(),
        vec![constant::ALLOCATOR_FILE],
        left_f
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
            .join(constant::ALLOCATOR_DIR)
            .to_str()
            .unwrap(),
        vec![constant::ALLOCATOR_FILE],
        left_f
    );

    log::info!("[Command Dealloc] Generating...");
    gen.gen(Path::new("./tmp"));
    log::info!("[Command Dealloc] End generating");
}