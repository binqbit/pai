use serde::{Deserialize, Serialize};

use crate::chatgpt::prompts::SUGGESTIONS;
use crate::terminal::Suggestion;
use crate::chatgpt::{ChatGPT, Message, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct Suggestions {
    suggestions: Vec<Suggestion>,
}

impl ChatGPT {
    pub fn get_suggestions(&self, cmd: String) -> Result<Vec<Suggestion>> {
        self.send_messages::<Suggestions>(vec![
            Message::new("system", SUGGESTIONS.to_owned()),
            Message::new("user", cmd),
        ]).map(|suggestions| suggestions.suggestions)
    }
}

impl ChatGPT {
    pub fn for_suggestions() -> Self {
        Self::from(Some(0.8), None, true)
    }
}