use std::io::{self, Write};
use crossterm::{cursor, style::{Color, PrintStyledContent, Stylize}, terminal::{Clear, ClearType}, ExecutableCommand};

use crate::terminal::colorize_command;
use super::{suggestions, Suggestions};



pub struct CommandBuffer {
    buffer: String,
}

impl CommandBuffer {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    pub fn push(&mut self, c: char) {
        if c == ' ' {
            if self.buffer.is_empty() {
                return;
            }
            if self.buffer.ends_with(' ') {
                return;
            }
        }
        self.buffer.push(c);
    }

    pub fn pop(&mut self) -> Option<char> {
        self.buffer.pop()
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn buffer(&self) -> &str {
        &self.buffer
    }

    pub fn print(&self, suggestions: &Suggestions) {
        let stdout = io::stdout();
        let mut stdout = stdout.lock();
        print!("\r{}", Clear(ClearType::CurrentLine));
        print!("> ");
        let original = self.buffer();
        let suggestion = suggestions.get_suggestion(original);
        stdout.execute(PrintStyledContent(colorize_command(&original).with(Color::White))).unwrap();
        print!(" ");
        stdout.execute(PrintStyledContent(suggestion.with(Color::DarkGrey))).unwrap();
        stdout.execute(cursor::MoveLeft(suggestion.len() as u16 + 1)).unwrap();
        stdout.flush().unwrap();
    }

    pub fn apply_suggestion(&mut self, suggestions: &Suggestions) {
        let suggestion = suggestions.get_current_suggestion();
        if !suggestion.is_empty() {
            if self.buffer.len() < suggestion.len() {
                let (buf, sug) = suggestion.split_at(self.buffer.len());
                if let Some(i) = sug.find(' ') {
                    self.buffer = format!("{buf}{}", sug[..i].to_string());
                } else {
                    self.buffer = suggestion.to_string();
                }
            }
        }
        self.push(' ');
    }

    pub fn apply_command(&mut self, suggestions: &Suggestions) {
        self.buffer = suggestions.get_current_command().to_string();
        self.push(' ');
    }
}