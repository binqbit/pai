use serde::{Serialize, Deserialize};
use serde_json::json;

use crate::{chatgpt::{Error, Function}, utils::FilePath};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Args {
    path: String,
}

lazy_static! {
    pub static ref FUNCTION_GET_LIST_DIR: Function = function!(
        "get_list_dir".to_owned(),
        json!({
            "name": "get_list_dir",
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

            let list_dirs = dirs
                .iter()
                .map(|dir| format!("'{dir}'"))
                .collect::<Vec<String>>()
                .join(", ");

            println!("[get_list_dir]('{}'): {}", args.path, list_dirs);

            Ok(serde_json::to_string(&dirs).unwrap())
        }
    );
}