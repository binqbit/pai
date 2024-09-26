use serde::{Serialize, Deserialize};
use serde_json::json;

use crate::{chatgpt::{Error, Function}, utils::FilePath};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Args {
    path: String,
}

lazy_static! {
    pub static ref FUNCTION_READ_FILE: Function = function!(
        "read_file".to_owned(),
        json!({
            "name": "read_file",
            "description": "Read a file content",
            "strict": true,
            "parameters": {
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "The path to the file"
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
            let (content, _line, tokens) = FilePath::new(&args.path)
                .map_err(|e| Error::ExecuteFunction(e.to_string()))?
                .read_file()
                .map_err(|e| Error::ExecuteFunction(e.to_string()))?
                .split("\n")
                .fold((String::new(), 1, 0), |(mut content, mut number, mut tokens), line| {
                    if tokens < 5000 {
                        tokens += tiktoken::count_text("gpt-4", line);
                        content.push_str(&format!("{}: {}\n", number, line));
                        number += 1;
                    }
                    (content, number, tokens)
                });

            println!("[read_file]('{}'): {} tokens", args.path, tokens);

            Ok(content)
        }
    );
}