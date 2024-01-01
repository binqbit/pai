use std::{process::Command, thread, env::consts::{OS, ARCH}};

use crate::{colorize_command, colorize_logs, read_database, ChatGPT, Message, list_files, Shell};



pub fn exec(commands: Vec<Shell>) {
    let thread_handle = thread::spawn(move || {
        for cmd in commands {
            if cmd.is_command {
                let cmd = cmd.content;
                println!("> {}", colorize_command(&cmd));
                if cmd.starts_with("cd") {
                    let path = cmd[3..].trim();
                    std::env::set_current_dir(path).expect("Failed to change directory");
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

                if let (status, Some(code)) = (status.success(), status.code()) {
                    let result = format!("{}, status code: {code}", if status { "success" } else { "failed" });
                    eprintln!("{}", colorize_logs(&result));
                }
            } else {
                println!("{}", colorize_logs(&cmd.content));
            }
        }
    });
    thread_handle.join().map_err(|_| "Failed to join thread").unwrap();
}

pub fn pai_run(gpt: &ChatGPT, command: String) {
    let path = std::env::current_dir().unwrap().display().to_string();
    let files = list_files(&path);
    let database = read_database();
    let messages = vec![
        Message::new("system", format!(r#"
os info: {OS} {ARCH}

current directory: {path}
files and directories:
{files}

additional commands:
{database}
"#)),
        Message::new("assistant", format!(r#"
write shell commands to execute as per user requirement: {command}
example:
    user: create new node js project 'my-project'
    assistant:
    ```shell
    mkdir my-project
    cd my-project
    npm init -y
    ```
"#)),
    ];

    match gpt.send(messages) {
        Ok(output) => {
            if let Some(commands) = output.shell() {
                exec(commands);
            } else if let Some(text) = output.text() {
                println!("{}", colorize_logs(&text));
            } else {
                println!("No response from ChatGPT");
            }
        },
        Err(err) => {
            eprintln!("Failed to get response from ChatGPT: {err}");
        },
    }
}