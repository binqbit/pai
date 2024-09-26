use std::{process::Command, thread};

use serde::{Serialize, Deserialize};
use serde_json::json;

use crate::{chatgpt::Function, terminal::{colorize_command, colorize_logs}};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Args {
    command: String,
}

lazy_static! {
    pub static ref FUNCTION_EXEC: Function = function!(
        "exec".to_owned(),
        json!({
            "name": "exec",
            "description": "Executes a command in the terminal",
            "strict": true,
            "parameters": {
                "type": "object",
                "properties": {
                    "command": {
                        "type": "string",
                        "description": "The command to run, e.g., 'ls -l'"
                    }
                },
                "additionalProperties": false,
                "required": [
                    "command"
                ]
            }
        }),
        |_chatgpt, arguments| {
            let args: Args = arguments.parse()?;

            println!("[exec]('{}'):", colorize_command(&args.command));

            exec(args.command);
            Ok(String::from("Success"))
        }
    );
}

pub fn exec(cmd: String) {
    let thread_handle = thread::spawn(move || {
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
        println!();
    });
    thread_handle.join().map_err(|_| "Failed to join thread").unwrap();
}