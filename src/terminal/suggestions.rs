use std::io::{self, Write};
use crossterm::{style::{Color, PrintStyledContent, Stylize}, terminal::{Clear, ClearType}, ExecutableCommand};
use serde::{Deserialize, Serialize};

use crate::terminal::colorize::colorize_command;



#[derive(Debug, Serialize, Deserialize)]
pub struct Suggestion {
    pub suggestion: String,
    pub command: String,
}

pub struct Suggestions {
    pub suggestions: Vec<Suggestion>,
    pub current_suggestion: usize,
}

impl Suggestions {
    pub fn new() -> Self {
        Self {
            suggestions: vec![
                Suggestion {
                    suggestion: "pai --help".to_string(),
                    command: "pai --help".to_string(),
                }
            ],
            current_suggestion: 0,
        }
    }

    pub fn set_suggestions(&mut self, suggestions: Vec<Suggestion>) {
        self.suggestions = suggestions;
        self.current_suggestion = 0;
        self.print_suggestions();
    }

    pub fn get_current_suggestion(&self) -> &str {
        return &self.suggestions[self.current_suggestion].suggestion;
    }

    pub fn get_current_command(&self) -> &str {
        return &self.suggestions[self.current_suggestion].command;
    }

    pub fn get_suggestion(&self, original: &str) -> &str {
        let suggestion = self.get_current_suggestion();
        if original.len() < suggestion.len() {
            &suggestion[original.len()..]
        } else {
            ""
        }
    }

    pub fn print_suggestions(&self) {
        let stdout = io::stdout();
        let mut stdout = stdout.lock();
        print!("{}{}", "\r".repeat(self.suggestions.len()), Clear(ClearType::All));
        for (i, suggestion) in self.suggestions.iter().enumerate() {
            if i == self.current_suggestion {
                stdout.execute(PrintStyledContent(">> ".with(Color::Blue))).unwrap();
            } else {
                print!("{}: ", i + 1);
            }
            println!("{}", colorize_command(&suggestion.command));
        }
        stdout.flush().unwrap();
    }

    pub fn next_suggestion(&mut self) {
        if self.current_suggestion < self.suggestions.len() - 1 {
            self.current_suggestion += 1;
            self.print_suggestions();
        }
    }

    pub fn prev_suggestion(&mut self) {
        if self.current_suggestion > 0 {
            self.current_suggestion -= 1;
            self.print_suggestions();
        }
    }
}