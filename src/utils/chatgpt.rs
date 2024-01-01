use reqwest::blocking::Client;
use serde::{Serialize, Deserialize};

use crate::{PAI_GPT_MODEL, Config};



pub type GptResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;


pub struct ChatGPT {
    apikey: String,
    model: String,
    client: Client,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatInput {
    model: String,
    messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatOutput {
    id: String,
    choices: Vec<ChoicesOutput>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChoicesOutput {
    message: MessageOutput,
}

#[derive(Debug, Serialize, Deserialize)]
struct MessageOutput {
    content: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Shell {
    pub is_command: bool,
    pub content: String,
}



impl ChatGPT {
    pub fn new(config: &Config) -> Self {
        Self {
            apikey: config.apikey.to_owned(),
            model: String::from(PAI_GPT_MODEL),
            client: Client::new(),
        }
    }

    pub fn send(&self, messages: Vec<Message>) -> GptResult<ChatOutput> {
        let input = serde_json::to_string(&ChatInput {
            model: self.model.clone(),
            messages,
        })?;

        let output = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", &format!("Bearer {}", self.apikey))
            .header("Content-Type", "application/json")
            .body(input)
            .send()?
            .error_for_status()?
            .json::<ChatOutput>()?;

        Ok(output)
    }
}

impl Message {
    pub fn new(role: &str, content: String) -> Self {
        Self {
            role: role.to_owned(),
            content,
        }
    }
}

impl ChatOutput {
    fn message(&self) -> Option<&MessageOutput> {
        if let Some(choice) = self.choices.get(0) {
            return Some(&choice.message);
        }
        None
    }

    pub fn text(&self) -> Option<String> {
        if let Some(message) = self.message() {
            if let Some(content) = &message.content {
                return Some(content.to_owned());
            }
        }
        None
    }

    pub fn json<T>(&self) -> Option<T>
    where
        T: serde::de::DeserializeOwned,
     {
        if let Some(text) = self.text() {
            if let Ok(json) = serde_json::from_str(&text) {
                return Some(json);
            } else {
                let start_json = text.find("```json");
                let end_json = text.rfind("```");
                if let (Some(start), Some(end)) = (start_json, end_json) {
                    let json = &text[start + 7..end];
                    if let Ok(json) = serde_json::from_str(json) {
                        return Some(json);
                    }
                }
            }
        }
        None
    }

    pub fn shell(&self) -> Option<Vec<Shell>> {
        if let Some(text) = self.text() {
            let mut commands = vec![];
            for shell in text.split("```shell") {
                if let Some(end_shell) = shell.find("```") {
                    let shell = shell.split_at(end_shell);
                    let text = shell.1[3..].trim();
                    let mut shell = shell.0.trim().to_string();
                    while let Some(_) = shell.find("\n\n") {
                        shell = shell.replace("\n\n", "\n");
                    }
                    for cmd in shell.split("\n") {
                        commands.push(Shell {
                            is_command: true,
                            content: cmd.trim().to_owned(),
                        });
                    }
                    if !text.is_empty() {
                        commands.push(Shell {
                            is_command: false,
                            content: text.to_owned(),
                        });
                    }
                } else {
                    commands.push(Shell {
                        is_command: false,
                        content: shell.trim().to_owned(),
                    });
                }
            }
            return Some(commands);
        }
        None
    }
}