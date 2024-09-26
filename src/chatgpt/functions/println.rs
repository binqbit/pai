use serde::{Serialize, Deserialize};
use serde_json::json;

use crate::{chatgpt::Function, terminal::colorize_logs};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Args {
    text: String,
}

lazy_static! {
    pub static ref FUNCTION_PRINTLN: Function = function!(
        "println".to_owned(),
        json!({
            "name": "println",
            "description": "Prints a message to the console",
            "strict": true,
            "parameters": {
                "type": "object",
                "properties": {
                    "text": {
                        "type": "string",
                        "description": "The text to print"
                    }
                },
                "additionalProperties": false,
                "required": [
                    "text"
                ]
            }
        }),
        |_chatgpt, arguments| {
            let args: Args = arguments.parse()?;
            println!("{}", colorize_logs(&args.text));
            Ok(String::from("Success"))
        }
    );
}