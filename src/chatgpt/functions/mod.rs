use serde::{Serialize, Deserialize, de::DeserializeOwned};
use serde_json::{json, Value};
use super::{ChatGPT, Message, Error, Result};



#[macro_export]
macro_rules! function {
    ($name:expr, $format:expr, $function:expr) => {
        {
            Function {
                name: $name,
                format: $format,
                function: $function,
            }
        }
    };
}

pub use function;



pub struct FunctionArgs(pub String);

pub struct Function {
    pub name: String,
    pub format: Value,
    pub function: fn(&ChatGPT, FunctionArgs) -> Result<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionInput {
    pub name: String,
    pub arguments: String,
}



impl Function {
    pub fn run(&self, gpt: &ChatGPT, values: FunctionArgs) -> Result<String> {
        (self.function)(gpt, values)
    }
}

impl FunctionArgs {
    pub fn parse<T: DeserializeOwned>(&self) -> Result<T> {
        serde_json::from_str(&self.0).map_err(|_| Error::ParseFunction(self.0.clone()))
    }
}



mod println;
mod exec;
mod get_dir_list;
mod get_dir_tree;
mod read_file;
mod write_file;

lazy_static! {
    pub static ref FUNCTIONS: Vec<&'static Function> = vec![
        &println::FUNCTION_PRINTLN,
        &exec::FUNCTION_EXEC,
        &get_dir_list::FUNCTION_GET_LIST_DIR,
        &get_dir_tree::FUNCTION_GET_TREE_DIR,
        &read_file::FUNCTION_READ_FILE,
        &write_file::FUNCTION_WRITE_FILE,
    ];

    pub static ref TOOLS: Vec<Value> = FUNCTIONS.iter().map(|f| json!({
        "type": "function",
        "function": f.format.clone(),
    })).collect();
}