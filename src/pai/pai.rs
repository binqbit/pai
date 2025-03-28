use std::{process::Command, thread};

use crate::{chatgpt::ChatGPT, terminal::{colorize_command, colorize_logs}, utils::print_markdown};

pub fn exec(commands: Vec<String>) {
    let thread_handle = thread::spawn(move || {
        for cmd in commands {
            println!("{} {}", colorize_logs(">"), colorize_command(&cmd));
            if cmd.starts_with("cd") {
                let path = cmd[3..].trim();
                std::env::set_current_dir(path).expect("Failed to change directory");
                println!();
                continue;
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
            println!();
        }
    });
    thread_handle.join().map_err(|_| "Failed to join thread").unwrap();
}

pub fn run_task(task: String) {
    match ChatGPT::for_assistant().run_assistant(task) {
        Ok(text) => {
            print_markdown(&text);
        },
        Err(err) => {
            eprintln!("Error: {err:?}");
        },
    }
}
