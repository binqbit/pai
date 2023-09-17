use std::collections::HashMap;

use serde_json::{json, Value};

use crate::{Function, Functions, function, run_commands, print_text, read_file, write_file, edit_text, ChatGPT, list_dirs};


lazy_static! {
    pub static ref FUNCTION_RUN_COMMANDS: Function = function!(
        json!({
            "name": "run_commands",
            "description": "Run commands in the terminal",
            "parameters": {
                "type": "object",
                "properties": {
                    "commands": {
                        "type": "array",
                        "description": "The array of commands to run, e.g.",
                        "items": {
                            "type": "string",
                            "description": "The command to run, e.g. echo Hello, world!"
                        },
                    },
                },
                "required": ["commands"],
            },
        }),
        |gpt: &ChatGPT, values: HashMap<String, Value>| {
            let commands = values.get("commands")
                .unwrap()
                .as_array()
                .unwrap()
                .iter()
                .map(|value| value.as_str().unwrap().to_owned())
                .collect::<Vec<_>>();
            run_commands(commands);
            None
        }
    );

    pub static ref FUNCTION_PRINT_TEXT: Function = function!(
        json!({
            "name": "print_text",
            "description": "Print text information",
            "parameters": {
                "type": "object",
                "properties": {
                    "texts": {
                        "type": "array",
                        "description": "The array of text to print, e.g.",
                        "items": {
                            "type": "string",
                            "description": "The text to print, e.g. Hello, world!"
                        },
                    },
                },
                "required": ["texts"],
            },
        }),
        |gpt: &ChatGPT, values: HashMap<String, Value>| {
            let texts = values.get("texts")
                .unwrap()
                .as_array()
                .unwrap()
                .iter()
                .map(|value| value.as_str().unwrap().to_owned())
                .collect::<Vec<_>>();
            print_text(texts);
            None
        }
    );

    pub static ref FUNCTION_READ_FILE: Function = function!(
        json!({
            "name": "read_file",
            "description": "Read a file contents",
            "parameters": {
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "The name of the file to create, e.g. file.txt",
                    },
                },
                "required": ["name"],
            },
            "output": {
                "type": "string",
                "description": "The contents of the file to create, e.g. Hello, world!",
            }
        }),
        |gpt: &ChatGPT, values: HashMap<String, Value>| {
            let name = values.get("name")
                .unwrap()
                .as_str()
                .unwrap()
                .to_owned();
            Some(Value::String(read_file(name)))
        }
    );

    pub static ref FUNCTION_WRITE_FILE: Function = function!(
        json!({
            "name": "write_file",
            "description": "Write a file",
            "parameters": {
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "The name of the file to create, e.g. file.txt",
                    },
                    "content": {
                        "type": "string",
                        "description": "The contents of the file to create, e.g. Hello, world!",
                    }
                },
                "required": ["name", "content"],
            }
        }),
        |gpt: &ChatGPT, values: HashMap<String, Value>| {
            let name = values.get("name")
                .unwrap()
                .as_str()
                .unwrap()
                .to_owned();
            let content = values.get("content")
                .unwrap()
                .as_str()
                .unwrap()
                .to_owned();
            write_file(name, content);
            None
        }
    );

    pub static ref FUNCTION_EDIT_TEXT: Function = function!(
        json!({
            "name": "edit_text",
            "description": "Edit text",
            "parameters": {
                "type": "object",
                "properties": {
                    "text": {
                        "type": "string",
                        "description": "The text to edit, e.g. Hello, world!",
                    },
                    "description": {
                        "type": "string",
                        "description": "A description of what needs to be changed or added to the text, e.g. Add a new line at the end of the text",
                    }
                },
                "required": ["text", "description"],
            },
            "output": {
                "type": "string",
                "description": "The edited text, e.g. Hello, world!\nAdd a new line at the end of the text",
            },
        }),
        |gpt: &ChatGPT, values: HashMap<String, Value>| {
            let text = values.get("text")
                .unwrap()
                .as_str()
                .unwrap()
                .to_owned();
            let description = values.get("description")
                .unwrap()
                .as_str()
                .unwrap()
                .to_owned();
            if let Some(res) = edit_text(gpt, text, description) {
                return Some(Value::String(res));
            } else {
                return None;
            }
        }
    );

    pub static ref FUNCTION_LIST_DIRS: Function = function!(
        json!({
            "name": "list_dirs",
            "description": "Get directories and files list",
            "parameters": {
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "The paths to get list, e.g. './'",
                    },
                },
                "required": ["path"],
            },
            "output": {
                "type": "array",
                "items": {
                    "type": "string",
                    "description": "The directory or file name, e.g."
                },
            },
        }),
        |gpt: &ChatGPT, values: HashMap<String, Value>| {
            let path = values.get("path")
                .unwrap()
                .as_str()
                .unwrap()
                .to_owned();
            Some(Value::Array(list_dirs(path).into_iter().map(|path| Value::String(path)).collect::<Vec<_>>()))
        }
    );

    pub static ref FUNCTIONS: Functions = Functions::new(vec![
        FUNCTION_RUN_COMMANDS.to_owned(),
        FUNCTION_PRINT_TEXT.to_owned(),
        FUNCTION_READ_FILE.to_owned(),
        FUNCTION_WRITE_FILE.to_owned(),
        FUNCTION_EDIT_TEXT.to_owned(),
        FUNCTION_LIST_DIRS.to_owned(),
    ]);
}