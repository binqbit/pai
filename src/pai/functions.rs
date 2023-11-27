use std::collections::HashMap;

use serde_json::{json, Value};

use crate::{ChatGPT, Function, Functions, function, execute_commands, print_text, read_file, write_file, list_dirs};


lazy_static! {
    pub static ref FUNCTION_EXECUTE_COMMANDS: Function = function!(
        json!({
            "name": "execute_commands",
            "description": "Execute the commands in the terminal",
            "parameters": {
                "type": "object",
                "properties": {
                    "commands": {
                        "type": "array",
                        "description": "The array of commands to execute, e.g. ['cd dir', 'ls']",
                        "items": {
                            "type": "string",
                            "description": "The command to execute, e.g. echo Hello, world!"
                        },
                    },
                },
                "required": ["commands"],
            },
        }),
        |_gpt: &ChatGPT, values: HashMap<String, Value>, history: &mut Vec<String>| {
            let commands = values.get("commands")
                .unwrap()
                .as_array()
                .unwrap()
                .iter()
                .map(|value| value.as_str().unwrap().to_owned())
                .collect::<Vec<_>>();
            let func = format!("execute_commands({commands:?})");
            if history.contains(&func) {
                return None;
            }
            history.push(func);
            Some(Value::String(execute_commands(commands)))
        }
    );

    pub static ref FUNCTION_PRINT_TEXT: Function = function!(
        json!({
            "name": "print_text",
            "description": "Print text information",
            "parameters": {
                "type": "object",
                "properties": {
                    "text": {
                        "type": "string",
                        "description": "The text to print, e.g. Hello, world!"
                    },
                },
                "required": ["text"],
            },
        }),
        |_gpt: &ChatGPT, values: HashMap<String, Value>, history: &mut Vec<String>| {
            let text = values.get("text")
                .unwrap()
                .as_str()
                .unwrap();
            let func = format!("print_text({text:?})");
            if history.contains(&func) {
                return None;
            }
            history.push(func);
            Some(Value::String(print_text(text)))
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
        |_gpt: &ChatGPT, values: HashMap<String, Value>, history: &mut Vec<String>| {
            let name = values.get("name")
                .unwrap()
                .as_str()
                .unwrap()
                .to_owned();
            let func = format!("read_file({name})");
            if history.contains(&func) {
                return None;
            }
            history.push(func);
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
        |_gpt: &ChatGPT, values: HashMap<String, Value>, history: &mut Vec<String>| {
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
            let func = format!("write_file({name})");
            if history.contains(&func) {
                return None;
            }
            history.push(func);
            Some(Value::String(write_file(name, content)))
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
        |_gpt: &ChatGPT, values: HashMap<String, Value>, history: &mut Vec<String>| {
            let path = values.get("path")
                .unwrap()
                .as_str()
                .unwrap()
                .to_owned();
            let func = format!("list_dirs({path})");
            if history.contains(&func) {
                return None;
            }
            history.push(func);
            Some(Value::String(list_dirs(path)))
        }
    );

    pub static ref FUNCTIONS: Functions = Functions::new(vec![
        FUNCTION_EXECUTE_COMMANDS.to_owned(),
        FUNCTION_PRINT_TEXT.to_owned(),
        FUNCTION_READ_FILE.to_owned(),
        FUNCTION_WRITE_FILE.to_owned(),
        FUNCTION_LIST_DIRS.to_owned(),
    ]);
}