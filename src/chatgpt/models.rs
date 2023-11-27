use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    content: Value,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub format: Value,
    pub function: fn(&ChatGPT, HashMap<String, Value>, &mut Vec<String>) -> Option<Value>,
}

#[derive(Debug, Clone)]
pub struct Functions (Vec<Function>);



impl Message {
    pub fn new(role: String, name: Option<String>, content: String) -> Self {
        Self {
            role,
            name,
            content: Value::String(content),
        }
    }
}

impl Function {
    pub fn new(format: Value, function: fn(&ChatGPT, HashMap<String, Value>, &mut Vec<String>) -> Option<Value>) -> Self {
        Self {
            format,
            function,
        }
    }

    pub fn get_name(&self) -> String {
        self.format
            .get("name")
            .expect("Missing name in function format.")
            .as_str()
            .expect("Invalid name in function format.")
            .to_string()
    }

    pub fn run(&self, gpt: &ChatGPT, values: HashMap<String, Value>, history: &mut Vec<String>) -> Option<Message> {
        if let Some(res) = (self.function)(gpt, values, history) {
            Some(Message {
                role: String::from("function"),
                name: Some(self.get_name()),
                content: res,
            })
        } else {
            None
        }
    }
}

impl Functions {
    pub fn new(functions: Vec<Function>) -> Self {
        Self(functions)
    }

    pub fn get_names(&self) -> Vec<String> {
        self.0.iter()
            .map(|function| function.get_name())
            .collect()
    }

    pub fn get(&self, name: &str) -> Option<&Function> {
        for function in &self.0 {
            if let Some(function_name) = function.format.get("name") {
                if let Some(function_name) = function_name.as_str() {
                    if function_name == name {
                        return Some(function);
                    }
                }
            }
        }
        None
    }

    pub fn run(&self, gpt: &ChatGPT, name: &str, values: HashMap<String, Value>, history: &mut Vec<String>) -> Option<Message> {
        if let Some(function) = self.get(name) {
            function.run(gpt, values, history)
        } else {
            None
        }
    }

    pub fn functions(&self) -> Vec<Value> {
        self.0.iter()
            .map(|function| function.format.to_owned())
            .collect()
    }
}



#[macro_export]
macro_rules! function {
    ($format:expr, $function:expr) => {
        {
            Function {
                format: $format,
                function: $function,
            }
        }
    };
}

pub use function;

use crate::ChatGPT;