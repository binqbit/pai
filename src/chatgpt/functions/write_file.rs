use serde::{Serialize, Deserialize};
use serde_json::json;

use crate::{chatgpt::{Error, Function}, utils::FilePath};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Args {
    path: String,
    content: String,
}

lazy_static! {
    pub static ref FUNCTION_WRITE_FILE: Function = function!(
        "write_file".to_owned(),
        json!({
            "name": "write_file",
            "description": "Write content to a file",
            "strict": true,
            "parameters": {
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "The path to the file"
                    },
                    "content": {
                        "type": "string",
                        "description": "The content to write to the file"
                    }
                },
                "additionalProperties": false,
                "required": [
                    "path",
                    "content"
                ]
            }
        }),
        |_chatgpt, arguments| {
            let args: Args = arguments.parse()?;
            FilePath::new(&args.path)
                .map_err(|e| Error::ExecuteFunction(e.to_string()))?
                .write_file(&args.content)
                .map_err(|e| Error::ExecuteFunction(e.to_string()))?;

            let content_size = if args.content.len() > 1000000 {
                format!("{}mb", args.content.len())
            } else if args.content.len() > 1000 {
                format!("{}kb", args.content.len())
            } else {
                format!("{}b", args.content.len())
            };
            println!("[write_file]('{}', {})", args.path, content_size);

            Ok(String::from("Success"))
        }
    );
}