use std::process::{Command, Stdio};
use std::thread;
use std::io::{BufRead, BufReader};
use serde::{Serialize, Deserialize};
use serde_json::json;
use crate::{chatgpt::Function, terminal::colorize_command};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Args {
    command: String,
}

lazy_static! {
    pub static ref FUNCTION_EXEC: Function = function!(
        "exec".to_owned(),
        json!({
            "name": "exec",
            "description": "Executes a command in the terminal and returns the logs.",
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

            let logs = exec(args.command);
            Ok(logs)
        }
    );
}

pub fn exec(cmd: String) -> String {
    let (sender, receiver) = std::sync::mpsc::channel();
    let thread_handle = thread::spawn(move || {
        if cmd.starts_with("cd") {
            let path = cmd[3..].trim();
            std::env::set_current_dir(path).expect("Failed to change directory");
            println!();
            return String::new();
        }
    
        let mut child = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .arg("/C")
                .arg(cmd)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("Failed to execute command")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("Failed to execute command")
        };
    
        let stdout = child.stdout.take().expect("Failed to capture stdout");
        let stderr = child.stderr.take().expect("Failed to capture stderr");
    
        let stdout_reader = BufReader::new(stdout);
        let stderr_reader = BufReader::new(stderr);
    
        for line in stdout_reader.lines().chain(stderr_reader.lines()) {
            match line {
                Ok(line) => {
                    println!("{}", &line);
                    sender.send(line).expect("Failed to send log line");
                }
                Err(err) => {
                    eprintln!("Failed to read line: {err}");
                }
            }
        }
    
        let status = child.wait().expect("Failed to wait for command");
        if !status.success() {
            eprintln!("Command exited with status: {status}");
        }
        println!();
        String::new()
    });

    thread_handle.join().map_err(|_| "Failed to join thread").unwrap();

    let logs = receiver.iter().collect::<Vec<String>>().join("\n");
    logs
}
