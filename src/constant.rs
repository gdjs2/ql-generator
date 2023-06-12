// The path to the qls directory
pub const QLS_PATH: &str = "./ql-generator-qls";

// Alloc Task directory
pub const ALLOCATOR_DIR: &str = "./alloc";
// Dealloc Task directory
pub const DEALLOCATOR_DIR: &str = "./dealloc";
// QL file for selecting all the allocators
pub const ALLOCATOR_FILE: &str = "./allocator.ql";
// QL file for selecting all the deallocators
pub const DEALLOCATOR_FILE: &str = "./deallocator.ql";

// Pattern key in the pattern ql files
pub const PATTERN_KEY: &str = "$${}$$";

// Util QL file for selecting all the functions from a CodeQL database
pub const SELECT_FUNC_QL_FILE: &str = "./utils/select_func.ql";

// CodeQL Binary Path
pub const CODEQL_BIN: &str = "codeql";

// Directory for working
pub const WORK_DIR: &str = "./.work_dir";

// The result for selecting all the functions (bqrs file)
pub const SELECT_FUN_RESULT_BQRS: &str = "./select_func.bqrs";
// The result for selecting all the functions (json file)
pub const SELECT_FUN_RESULT_JSON: &str = "./select_func.json";

// URL for OpenAI Chat Completion 
pub const OPENAI_CHAT_COMPLETION_URL: &str = "https://api.openai.com/v1/chat/completions";
// Model used for Chat Completion
pub const OPENAI_CHAT_COMPLETION_MODEL: &str = "gpt-3.5-turbo";

// Prompt for selecting all allocators
pub const ALLOCATOR_PROMPT: &str = 
r#"You are a helpful assistant who help me to decide whether a function 
is an allocator who (allocates/reserves) a block of memory from computer
memory or not. 

You should give the answer ONLY with the json format, no other words: 
{"result": "(Yes or No)"}. 

You will receive the whole function definition and implementation."#;

// Promt for selecting all deallocators
pub const DEALLOCATOR_PROMPT: &str = 
r#"You are a helpful assistant who help me to decide whether a function 
is an deallocator who (deallocates/free) a block of memory from computer
memory or not. 

You should give the answer ONLY with the json format, no other words: 
{"result": "(Yes or No)"}. 

You will receive the whole function definition and implementation."#;

// Promt for selecting all deallocators
pub const DEALLOCATOR_ARG_PROMPT: &str = 
r#"You are a helpful assistant who help me to decide whether a function 
is an deallocator who (deallocates/free) a block of memory from computer
memory or not. Provide the index number (from 0) of argument effected by the 
deallocation as well. For example, in free(void *ptr), the index number
is 0, because the free function will deallocate the memory in ptr. Index
field is -1 if the result is No.

You should give the answer ONLY with the json format, no other words: 
{
	"result": "(Yes or No)"
	"index": "(index here, from 0)"
} 

You will receive the whole function definition and implementation."#;