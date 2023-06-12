use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use ql_generator::{
    constant,
    engine::{chatgpt::ChatGPTEngine, Engine},
    extractor::{codeql::CodeQLExtractor, Extractor},
    generator::{
        codeql::{CodeQLGenerator, Pts},
        Generator,
    },
};

use crate::{AllocArgs, DeallocArgs, UseAfterFreeArgs};

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

    // Extract functions
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

    // Generate QL Code
    log::info!("[Command Alloc] Creating CodeQL Code...");
    let mut ql = String::new();
    for f in left_f {
        ql.push_str(&format!("\t\tor fun.hasGlobalName(\"{}\")\n", f.name));
    }

    // Create the CodeQL Generator
    log::info!("[Command Alloc] Creating CodeQL Generator...");
    let gen = CodeQLGenerator::new(
        Path::new(constant::QLS_PATH)
            .join(constant::ALLOCATOR_DIR)
            .to_str()
            .unwrap(),
        vec![Pts {
            f: PathBuf::new().join(constant::ALLOCATOR_FILE),
            s: ql,
        }],
    );

    // Generate the target QL pack
    log::info!("[Command Alloc] Generating...");
    gen.gen(Path::new("./tmp"));
    log::info!("[Command Alloc] End generating");
}

/**
The task for generating ql files for allocator selecting.

* args: &[`AllocArgs`], which is the arguments for allocator task
*/
pub fn dealloc_task(args: &DeallocArgs) {
    // Create CodeQL Extractor
    log::info!(
        "[Command Dealloc] Creating CodeQL Extractor using database {}",
        &args.db
    );
    let extractor = CodeQLExtractor::new(args.db.clone());

    // Extract functions
    log::info!("[Command Dealloc] Extracting functions...");
    let funcs = extractor.extract_funcs();
    log::info!(
        "[Command Dealloc] Extracted {} functions in total",
        funcs.len()
    );

    // Create ChatGPT Engine
    log::info!("[Command Dealloc] Creating ChatGPT Engine...");
    let engine = ChatGPTEngine::new(std::env::var("OPENAI_KEY").unwrap());

    // Define the vector for the left functions after asking for Engine
    let mut left_f = Vec::new();

    // Ask for engine
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

    // Create CodeQL Generator
    // Generate QL Code
    log::info!("[Command Dealloc] Creating CodeQL Code...");
    let mut ql = String::new();
    for f in left_f {
        ql.push_str(&format!("\t\tor fun.hasGlobalName(\"{}\")\n", f.name));
    }

    // Create the CodeQL Generator
    log::info!("[Command Dealloc] Creating CodeQL Generator...");
    let gen = CodeQLGenerator::new(
        Path::new(constant::QLS_PATH)
            .join(constant::DEALLOCATOR_DIR)
            .to_str()
            .unwrap(),
        vec![Pts {
            f: PathBuf::new().join(constant::DEALLOCATOR_FILE),
            s: ql,
        }],
    );

    // Generate the target QL pack
    log::info!("[Command Dealloc] Generating...");
    gen.gen(Path::new("./tmp"));
    log::info!("[Command Dealloc] End generating");
}

pub fn use_after_free_task(args: &UseAfterFreeArgs) {
    // Create CodeQL Extractor
    log::info!(
        "[Command UAF] Creating CodeQL Extractor using database {}",
        &args.db
    );
    let extractor = CodeQLExtractor::new(args.db.clone());

    // Extract functions
    log::info!("[Command UAF] Extracting functions...");
    let funcs = extractor.extract_funcs();
    log::info!("[Command UAF] Extracted {} functions in total", funcs.len());

    // Create ChatGPT Engine
    log::info!("[Command UAF] Creating ChatGPT Engine...");
    let engine = ChatGPTEngine::new(std::env::var("OPENAI_KEY").unwrap());

    // Define the vector for the left functions after asking for Engine
    let mut left_f = Vec::new();
    let mut idx_v = Vec::new();

    // Ask for engine
    log::info!("[Command UAF] Start asking...");
    for f in &funcs {
        let (res, idx) = engine.is_deallocator_and_idx(f);
        if res {
            left_f.push(f);
            idx_v.push(idx);
        }
        log::info!(
            "[Command UAF] Function{{ {} }}, Result{{ {} }}. Index {{ {} }}",
            f.name,
            res,
            idx
        );
    }
    log::info!("[Command UAF] End asking, {} functions left", left_f.len());

    let mut v = Vec::new();
    let mut i = 0;
    loop {
        v.push((left_f[i], idx_v[i]));
        i += 1;
        if i == idx_v.len() {
            break;
        }
    }

    v.sort_by(|a, b| a.1.cmp(&b.1));

    // Create CodeQL Generator
    // Generate QL Code
    log::info!("[Command UAF] Creating CodeQL Code...");
    let mut ql = String::from_str("(\n(\n").unwrap();
    i = 0;
    loop {
        let e = &v[i];
        if i == 0 {
            ql.push_str(format!("fc.getTarget().hasGlobalOrStdName(\"{}\") ", e.0.name).as_str());
            if i != v.len() - 1 {
                let next = &v[i + 1];
                if e.1 == next.1 {
                    ql.push_str("or\n");
                } else {
                    if e.1 != -1 {
                        ql.push_str(format!(") and \nva = fc.getArgument({})\n) ", e.1).as_str());
                    } else {
                        ql.push_str(")\n) or\n");
                    }
                }
            } else {
                if e.1 != -1 {
                    ql.push_str(format!(") and \nva = fc.getArgument({})\n) ", e.1).as_str());
                } else {
                    ql.push_str(")\n)\n");
                }
            }
        } else {
            let last = &v[i - 1];
            if e.1 != last.1 {
                ql.push_str("(\n(\n");
            }

            ql.push_str(format!("fc.getTarget().hasGlobalOrStdName(\"{}\") ", e.0.name).as_str());
            if i != v.len() - 1 {
                let next = &v[i + 1];
                if e.1 == next.1 {
                    ql.push_str("or\n");
                } else {
                    if e.1 != -1 {
                        ql.push_str(format!(") and \nva = fc.getArgument({})\n) ", e.1).as_str());
                    } else {
                        ql.push_str(")\n) or\n");
                    }
                }
            } else {
                if e.1 != -1 {
                    ql.push_str(format!(") and \nva = fc.getArgument({})\n) ", e.1).as_str());
                } else {
                    ql.push_str(")\n)\n");
                }
            }
            
        }
        i += 1;
        if i == v.len() {
            break;
        }
    }

    // Create the CodeQL Generator
    log::info!("[Command UAF] Creating CodeQL Generator...");
    let gen = CodeQLGenerator::new(
        Path::new(constant::QLS_PATH)
            .join(constant::UAF_DIR)
            .to_str()
            .unwrap(),
        vec![Pts {
            f: PathBuf::new().join(constant::UAF_FILE),
            s: ql,
        }],
    );

    // Generate the target QL pack
    log::info!("[Command UAF] Generating...");
    gen.gen(Path::new("./tmp"));
    log::info!("[Command UAF] End generating");
}
