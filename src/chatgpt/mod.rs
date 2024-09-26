use reqwest::blocking::Client;
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use serde_json::{json, Value};
use crate::{utils::API_KEY, PAI_GPT_MODEL};



mod prompts;
mod config;
mod result;
mod commands;
mod functions;

pub use config::*;
pub use result::*;
pub use commands::*;
pub use functions::*;



pub struct ChatGPT {
    pub temperature: Option<f32>,
    pub tools: Option<Vec<Value>>,
    pub is_json: bool,
    pub client: Client,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: Option<Vec<Content>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_call_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Content {
    #[serde(rename = "type")]
    pub _type: String,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub function: FunctionInput,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionInput {
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Serialize, Clone)]
struct ChatInput<'a> {
    model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<Value>,
    messages: &'a Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parallel_tool_calls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<&'a Vec<Value>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatOutput {
    choices: Vec<Value>,
}



impl ChatGPT {
    pub fn from(temperature: Option<f32>, tools: Option<Vec<Value>>, is_json: bool) -> Self {
        Self {
            temperature,
            tools,
            is_json,
            client: Client::new(),
        }
    }

    pub fn send_messages<T: DeserializeOwned>(&self, mut messages: Vec<Message>) -> Result<T> {
        let output = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", &format!("Bearer {}", *API_KEY))
            .header("Content-Type", "application/json")
            .body(ChatInput::form_messages(&messages, self.temperature.clone(), self.tools.as_ref(), self.is_json).to_string())
            .send()
            .map_err(|err| {
                eprintln!("Error sending messages: {err:?}");
                Error::Request(err.to_string())
            })?
            .error_for_status()
            .map_err(|err| {
                eprintln!("Error with status: {err:?}");
                Error::Request(err.to_string())
            })?
            .json::<ChatOutput>()
            .map_err(|err| {
                eprintln!("Error parsing output: {err:?}");
                Error::Output(err.to_string())
            })?;

        if let Some(assistant) = output.tool_calls()? {
            messages.push(assistant.clone());
            for tool_call in assistant.tool_calls.unwrap() {
                messages.push(tool_call.run(self)?);
            }
            self.send_messages(messages)
        } else {
            if self.is_json {
                output.content()
            } else {
                output.result()
            }
        }
    }
}



impl Message {
    pub fn new(role: &str, content: String) -> Self {
        Self {
            role: role.to_owned(),
            content: Some(vec![Content {
                _type: "text".to_owned(),
                text: content,
            }]),
            tool_calls: None,
            tool_call_id: None,
        }
    }

    pub fn from<T>(role: &str, json: T) -> Self
    where
        T: Serialize,
    {
        Self {
            role: role.to_owned(),
            content: Some(vec![Content {
                _type: "json_object".to_owned(),
                text: serde_json::to_string(&json).unwrap(),
            }]),
            tool_calls: None,
            tool_call_id: None,
        }
    }

    pub fn tool_call(result: String, tool_call_id: String) -> Self {
        Self {
            role: "tool".to_owned(),
            content: Some(vec![Content {
                _type: "text".to_owned(),
                text: result,
            }]),
            tool_calls: None,
            tool_call_id: Some(tool_call_id),
        }
    }
}

impl<'a> ChatInput<'a> {
    pub fn form_messages(messages: &'a Vec<Message>, temperature: Option<f32>, tools: Option<&'a Vec<Value>>, is_json: bool) -> Self {
        Self {
            model: PAI_GPT_MODEL.to_owned(),
            temperature,
            response_format: if is_json {
                Some(json!({
                    "type": "json_object",
                }))
            } else {
                None
            },
            messages,
            parallel_tool_calls: if tools.is_some() { Some(true) } else { None },
            tools,
        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl ChatOutput {
    fn tool_calls(&self) -> Result<Option<Message>> {
        let choices = match self.choices.get(0) {
            Some(choice) => choice,
            None => return Ok(None),
        };
        if Some("tool_calls") != choices.get("finish_reason").and_then(|val| val.as_str()) {
            return Ok(None);
        }
        let message = match choices.get("message") {
            Some(message) => message,
            None => return Ok(None),
        };
        let tool_calls = match message.get("tool_calls") {
            Some(tool_calls) => tool_calls,
            None => return Ok(None),
        };
        if !tool_calls.is_null() {
            Ok(Some(
                serde_json::from_value(message.clone())
                    .map_err(|err| Error::Output(err.to_string()))?
            ))
        } else {
            Ok(None)
        }
    }

    fn result<T: DeserializeOwned>(&self) -> Result<T> {
        let choices = self.choices.get(0)
            .ok_or(Error::Output("Result not found".to_owned()))?;
        let message = choices.get("message")
            .ok_or(Error::Output("Result not found".to_owned()))?;
        let value = message.get("content")
            .ok_or(Error::Output("Result not found".to_owned()))?;
        Ok(serde_json::from_value(value.clone()).map_err(|err| Error::Output(err.to_string()))?)
    }

    fn content<T: DeserializeOwned>(&self) -> Result<T> {
        let content = self.result::<String>()?;
        serde_json::from_str(&content).map_err(|err| Error::Output(err.to_string()))
    }
}

impl ToolCall {
    pub fn run(&self, gpt: &ChatGPT) -> Result<Message> {
        let function = FUNCTIONS
            .iter()
            .find(|func| func.name == self.function.name)
            .ok_or(Error::ExecuteFunction("Function not found".to_owned()))?;
        let result = function.run(gpt, FunctionArgs(self.function.arguments.clone()))?;
        Ok(Message::tool_call(result, self.id.clone()))
    }
}