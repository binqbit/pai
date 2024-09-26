use serde::{Serialize, Deserialize};
use serde_json::json;

use crate::{chatgpt::{Error, Function}, utils::ProjectDir};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Args {
    path: String,
}

lazy_static! {
    pub static ref FUNCTION_GET_TREE_DIR: Function = function!(
        "get_dir_tree".to_owned(),
        json!({
            "name": "get_dir_tree",
            "description": "Get a tree of directories",
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
            
            println!("[get_dir_tree]('{}'):", args.path);
            
            let tree = ProjectDir::new().tree(&args.path, true)
                .map_err(|e| Error::ExecuteFunction(e.to_string()))?;
            Ok(tree)
        }
    );
}