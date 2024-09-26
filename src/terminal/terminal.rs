use chrono::Utc;
use crossterm::{event::{self, Event, KeyCode, KeyEvent, KeyModifiers}, execute, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}};
use once_cell::sync::OnceCell;
use std::{io::{self, Write}, process::Command, sync::Arc, thread};

use crate::{chatgpt::ChatGPT, terminal::{colorize_command, colorize_logs, suggestions::Suggestions, CommandBuffer}};


static mut suggestions: OnceCell<Suggestions> = OnceCell::new();
static mut buffer: OnceCell<CommandBuffer> = OnceCell::new();
static mut current_process: i64 = 0;


fn find_suggestions(chatgpt: Arc<ChatGPT>) {
    std::thread::spawn(move || {
        let process = Utc::now().timestamp_millis();
        unsafe {
            current_process = process;
            let list = chatgpt.get_suggestions(buffer.get().unwrap().buffer().to_string()).unwrap();
            if current_process != process {
                return;
            }
            suggestions.get_mut().unwrap().set_suggestions(list);
            buffer.get().unwrap().print(&suggestions.get().unwrap());
        }
    });
}


pub fn exec(cmd: String) {
    let thread_handle = thread::spawn(move || {
        println!("{} {}", colorize_logs(">"), colorize_command(&cmd));
        if cmd.starts_with("cd") {
            let path = cmd[3..].trim();
            std::env::set_current_dir(path).expect("Failed to change directory");
            println!();
            return;
        }
        let mut child = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .arg("/C")
                .arg(cmd)
                .spawn()
                .expect("Failed to execute command")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .spawn()
                .expect("Failed to execute command")
        };

        let status = child.wait().expect("Failed to wait for command");
        if !status.success() {
            eprintln!("Command exited with status: {status}");
        }
    });
    thread_handle.join().map_err(|_| "Failed to join thread").unwrap();
}


pub fn input_and_processing(flags: Vec<String>) -> std::io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    let chatgpt = Arc::new(ChatGPT::for_suggestions());
    unsafe {
        suggestions.set(Suggestions::new());
        buffer.set(CommandBuffer::new());
    }
    unsafe {
        suggestions.get_mut().unwrap().print_suggestions();
    }
    
    loop {
        unsafe {
            buffer.get().unwrap().print(&suggestions.get().unwrap());
        }
        if let Event::Key(KeyEvent { code, modifiers, kind, .. }) = event::read()? {
            if kind == event::KeyEventKind::Press {
                match code {
                    KeyCode::Char(c) => {
                        if modifiers.contains(KeyModifiers::CONTROL) && c == 'c' {
                            break;
                        } else {
                            unsafe {
                                buffer.get_mut().unwrap().push(c);
                            }
                        }
                        if c == ' ' {
                            find_suggestions(chatgpt.clone());
                        }
                    },
                    KeyCode::Tab => {
                        unsafe {
                            buffer.get_mut().unwrap().apply_suggestion(&suggestions.get().unwrap());
                            find_suggestions(chatgpt.clone());
                        }
                    },
                    KeyCode::Enter => {
                        unsafe {
                            exec(suggestions.get().unwrap().get_current_command().to_string());
                            buffer.get_mut().unwrap().clear();
                        }
                    },
                    KeyCode::Backspace => {
                        unsafe {
                            buffer.get_mut().unwrap().pop();
                        }
                        print!("\x08 \x08");
                        stdout.flush().unwrap();
                    },
                    KeyCode::Up => {
                        unsafe {
                            suggestions.get_mut().unwrap().prev_suggestion();
                        }
                    },
                    KeyCode::Down => {
                        unsafe {
                            suggestions.get_mut().unwrap().next_suggestion();
                        }
                    },
                    KeyCode::Right => {
                        unsafe {
                            buffer.get_mut().unwrap().apply_command(&suggestions.get().unwrap());
                            find_suggestions(chatgpt.clone());
                        }
                    },
                    _ => {
                    }
                }
            }
        }
    }

    terminal::disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;
    Ok(())
}
