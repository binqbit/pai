use std::env::consts::{ OS, ARCH };

use crate::chatgpt::prompts::ASSISTANT;
use crate::chatgpt::{ChatGPT, Message, Result, TOOLS};
use crate::utils::FilePath;


impl ChatGPT {
    pub fn run_assistant(&self, task: String) -> Result<String> {
        let os_info = format!("{} {}", OS, ARCH);
        let cur_dir = FilePath::current().get_path();
        self.send_messages::<String>(vec![
            Message::new("system", ASSISTANT.replace("{{os}}", &os_info).replace("{{cur_dir}}", &cur_dir)),
            Message::new("user", task),
        ])
    }
}

impl ChatGPT {
    pub fn for_assistant() -> ChatGPT {
        ChatGPT::from(Some(0.8), Some(TOOLS.clone()), false)
    }
}