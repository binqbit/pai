use serde::{Serialize, Deserialize};
use serde_json::json;

use crate::{chatgpt::Function, utils::print_markdown};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Args {
    content: String,
}

lazy_static! {
    pub static ref FUNCTION_PRINTLN: Function = function!(
        "print".to_owned(),
        json!({
            "name": "print",
            "description": "Print markdown formatted message to terminal",
            "strict": true,
            "parameters": {
                "type": "object",
                "properties": {
                    "content": {
                        "type": "string",
                        "description": "The content to print in markdown format"
                    },
                },
                "additionalProperties": false,
                "required": [
                    "content"
                ]
            }
        }),
        |_chatgpt, arguments| {
            let args: Args = arguments.parse()?;

            print_markdown(&args.content);

            Ok(String::from("Success"))
        }
    );
}