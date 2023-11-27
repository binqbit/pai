use reqwest::blocking::Client;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use rust_tokenizers::tokenizer::{Tokenizer, TruncationStrategy, Gpt2Tokenizer};

use crate::{Functions, Message, GptResult, Config, get_exec_path, History};

lazy_static! {
    pub static ref TOKENIZER: Gpt2Tokenizer = {
        let path = get_exec_path().join("config");
        let vocab = path.join("vocab.json");
        let merges = path.join("merges.txt");
        Gpt2Tokenizer::from_file(vocab, merges, true)
        .expect("Failed to create tokenizer")
    };
}

fn count_tokens(prompt: &str) -> usize {
    let encoding = TOKENIZER.encode(prompt, None, 3000, &TruncationStrategy::LongestFirst, 2);
    encoding.token_ids.len()
}



pub struct ChatGPT {
    apikey: String,
    model: String,
    client: Client,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatInput {
    model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    temperature: Option<f64>,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    functions: Option<Vec<Value>>,
}



#[derive(Debug, Serialize, Deserialize, Clone)]
struct FunctionInput {
    name: String,
    arguments: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct MessageOutput {
    content: Option<String>,
    function_call: Option<FunctionInput>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChoicesOutput {
    message: MessageOutput,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatOutput {
    pub id: String,
    pub choices: Vec<ChoicesOutput>,
}

impl ChatGPT {
    pub fn new(config: Config) -> Option<Self> {
        if let Some(apikey) = config.openai_key {
            Some(Self {
                apikey,
                model: config.gpt_model,
                client: Client::new(),
            })
        } else {
            None
        }
    }

    pub fn send(&self, mut messages: Vec<Message>, functions: Option<Functions>, history: &mut History, flags: Vec<String>) -> GptResult<Option<String>> {
        let input = serde_json::to_string(&ChatInput {
            model: self.model.to_owned(),
            max_tokens: None,
            temperature: Some(0.0),
            messages: messages.to_owned(),
            functions: functions.to_owned().map(|f| f.functions()),
        })?;

        let tokens = count_tokens(&input);
        if tokens > 4000 {
            panic!("Input is too long: {} tokens", tokens);
        }
        
        let input = serde_json::to_string(&ChatInput {
            model: self.model.to_owned(),
            max_tokens: Some(4000 - tokens as u32),
            temperature: None,
            messages: messages.to_owned(),
            functions: functions.to_owned().map(|f| f.functions()),
        })?;

        let output = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", &format!("Bearer {}", self.apikey))
            .header("Content-Type", "application/json")
            .body(input)
            .send()?
            .error_for_status()?
            .json::<ChatOutput>()?;

        if let Some(func) = output.function() {
            if let Some(funcs) = functions.as_ref() {
                let args = serde_json::from_str(&func.arguments)?;
                if let Some(message) = funcs.run(self, &func.name, args, history) {    
                    if !flags.contains(&String::from("-c")) {
                        messages.push(message);
                        return self.send(messages, functions, history, flags);
                    }
                } else {
                    return self.send(messages, functions, history, flags);
                }
            }
        }

        Ok(output.text())
    }
}



impl ChatOutput {
    fn message(&self) -> Option<&MessageOutput> {
        if let Some(choice) = self.choices.get(0) {
            return Some(&choice.message);
        }
        None
    }

    fn text(&self) -> Option<String> {
        if let Some(message) = self.message() {
            if let Some(content) = &message.content {
                return Some(content.to_owned());
            }
        }
        None
    }

    // fn json<T>(&self) -> Option<T>
    // where
    //     T: serde::de::DeserializeOwned,
    //  {
    //     if let Some(text) = self.text() {
    //         if let Ok(json) = serde_json::from_str(&text) {
    //             return Some(json);
    //         }
    //     }
    //     None
    // }

    fn function(&self) -> Option<FunctionInput> {
        if let Some(message) = self.message() {
            return message.function_call.to_owned();
        }
        None
    }
}