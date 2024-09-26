use serde::{Serialize, Deserialize};
use serde_json::json;

use crate::{chatgpt::{Error, Function}, utils::FilePath};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Args {
    path: String,
}

lazy_static! {
    pub static ref FUNCTION_GET_LIST_DIR: Function = function!(
        "get_dir_list".to_owned(),
        json!({
            "name": "get_dir_list",
            "description": "Get a list of directories",
            "strict": true,
            "parameters": {
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "The path to the directory"
                    }
                },
                "additionalProperties": false,
                "required": [
                    "path"
                ]
            }
        }),
        |_chatgpt, arguments| {
            let args: Args = arguments.parse()?;
            let dirs = FilePath::new(&args.path)
                .map_err(|e| Error::ExecuteFunction(e.to_string()))?
                .read_dir()
                .map_err(|e| Error::ExecuteFunction(e.to_string()))?;

            println!("[get_dir_list]('{}'): {}", args.path, dirs.join(", "));

            Ok(serde_json::to_string(&dirs).unwrap())
        }
    );
}