use std::{process::Command, thread};

use crate::{Message, ChatGPT, FUNCTIONS, colorize_command, colorize_logs};



pub fn execute_commands(commands: Vec<String>) {
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
                    eprintln!("{}", colorize_logs(&format!("{}, status code: {code}", if status { "success" } else { "failed" })));
                }
        }
    });
    thread_handle.join().expect("Failed to join thread");
}



pub fn print_text(text: &str) {
    println!("{}", colorize_logs(text));
}

pub fn read_file(name: String) -> String {
    println!("> read_file: {}", colorize_logs(&name));
    let contents = std::fs::read_to_string(name)
        .expect("Something went wrong reading the file");
    contents
}

pub fn write_file(name: String, content: String) {
    println!("> write_file: {}", name);
    std::fs::write(name, content)
        .expect("Something went wrong writing the file");
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

    match gpt.send(messages, Some(FUNCTIONS.to_owned())) {
        Ok(Some(res)) => {
            println!("{}", colorize_logs(&res));
        },
        Err(err) => {
            eprintln!("Failed to get response from ChatGPT: {}", err);
        },
        _ => {},
    }
}