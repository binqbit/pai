use std::{process::Command, thread};

use crate::{Message, ChatGPT, FUNCTIONS, colorize_command, colorize_logs};



pub fn execute_commands(commands: Vec<String>) -> String {
    let mut cmd_status = String::new();
    let thread_handle = thread::spawn(move || {
        for cmd in commands {
                println!("> {}", colorize_command(&cmd));
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
                    cmd_status = format!("{}\n>{}", cmd_status, result);
                    eprintln!("{}", colorize_logs(&result));
                }
        }
        cmd_status
    });
    thread_handle.join().map_err(|_| "Failed to join thread").unwrap()
}



pub fn print_text(text: &str) -> String {
    println!("{}", colorize_logs(text));
    "ok".to_string()
}

pub fn read_file(name: String) -> String {
    println!("> read_file: {}", colorize_logs(&name));
    let contents = std::fs::read_to_string(name)
        .expect("Something went wrong reading the file");
    contents
}

pub fn write_file(name: String, content: String) -> String {
    println!("> write_file: {}", name);
    if let Err(err) = std::fs::write(name, content) {
        eprintln!("Failed to write file: {}", err);
        format!("Failed to write file: {}", err)
    } else {
        "File written successfully".to_string()
    }
}

pub fn list_dirs(path: String) -> String {
    println!("> list_dirs: {}", colorize_logs(&path));
    std::fs::read_dir(path)
        .expect("Failed to read directory")
        .collect::<Vec<_>>()
        .iter()
        .map(|path| path.as_ref().unwrap().path().display().to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn pai_run(gpt: &ChatGPT, task: String) {
    let messages = vec![
        Message::new(String::from("user"), None, format!(r#"
os info: {} {}
current directory: {}
complete the user's task using the available functions: {}
"#,
    std::env::consts::OS,
    std::env::consts::ARCH,
    std::env::current_dir().unwrap().display(),
    task)),
    ];

    match gpt.send(messages, Some(FUNCTIONS.to_owned()), &mut vec![]) {
        Ok(Some(res)) => {
            println!("{}", colorize_logs(&res));
        },
        Err(err) => {
            eprintln!("Failed to get response from ChatGPT: {}", err);
        },
        _ => {},
    }
}