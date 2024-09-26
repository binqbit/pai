use std::env::consts::{ OS, ARCH };

use crate::chatgpt::prompts::COMMANDS;
use crate::chatgpt::{ChatGPT, Message, Result, TOOLS};
use crate::utils::FilePath;


impl ChatGPT {
    pub fn run_assistant(&self, task: String) -> Result<String> {
        let path = FilePath::current().get_path();
        self.send_messages::<>(vec![
            Message::new("system", COMMANDS.to_owned()),
            Message::new("user", format!(r#"
User System Info:
- os info: {OS} {ARCH}
- current directory: {path}

Task: {task}
"#)),
        ])
    }
}

impl ChatGPT {
    pub fn for_assistant() -> ChatGPT {
        ChatGPT::from(Some(0.8), Some(TOOLS.clone()), false)
    }
}