pub const QLS_PATH: &str = "./ql-generator-qls";
pub const ALLOCATOR_DIR: &str = "./alloc";
pub const DEALLOCATOR_DIR: &str = "./dealloc";
pub const ALLOCATOR_FILE: &str = "./allocator.ql";
pub const DEALLOCATOR_FILE: &str = "./deallocator.ql";

pub const PATTERN_KEY: &str = "$${}$$";

pub const SELECT_FUNC_QL_FILE: &str = "./utils/select_func.ql";

pub const CODEQL_BIN: &str = "codeql";

pub const WORK_DIR: &str = "./.work_dir";

pub const SELECT_FUN_RESULT_BQRS: &str = "./select_func.bqrs";
pub const SELECT_FUN_RESULT_JSON: &str = "./select_func.json";

pub const OPENAI_CHAT_COMPLETION_URL: &str = "https://api.openai.com/v1/chat/completions";
pub const OPENAI_CHAT_COMPLETION_MODEL: &str = "gpt-3.5-turbo";

pub const ALLOCATOR_PROMPT: &str = 
r#"You are a helpful assistant who help me to decide whether a function 
is an allocator who (allocates/reserves) a block of memory from computer
memory or not. 

You should give the answer ONLY with the json format, no other words: 
{"result": "(Yes or No)"}. 

You will receive the whole function definition and implementation."#;

pub const DEALLOCATOR_PROMPT: &str = 
r#"You are a helpful assistant who help me to decide whether a function 
is an deallocator who (deallocates/free) a block of memory from computer
memory or not. 

You should give the answer ONLY with the json format, no other words: 
{"result": "(Yes or No)"}. 

You will receive the whole function definition and implementation."#;