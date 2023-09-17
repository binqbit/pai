use std::{process::Command, thread};

use crate::{Message, ChatGPT, FUNCTIONS, colorize_command, colorize_logs};



pub fn run_commands(commands: Vec<String>) {
    let thread_handle = thread::spawn(move || {
        for cmd in commands {
                println!("> {}", colorize_command(&cmd));
                let mut child = Command::new("cmd")
                    .arg("/C")
                    .arg(cmd)
                    .spawn()
                    .expect("Failed to execute command");

                let status = child.wait().expect("Failed to wait for command");

                if let (false, Some(code)) = (status.success(), status.code()) {
                    eprintln!("{}", colorize_logs(&format!("status code error: {code}")));
                }
        }
    });
    thread_handle.join().expect("Failed to join thread");
}



pub fn print_text(texts: Vec<String>) {
    if texts.len() == 1 {
        println!("{}", colorize_logs(&texts[0]));
    } else if texts.len() > 1 {
        for text in texts {
            println!("{}", colorize_logs(&text));
        }
    }
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

pub fn edit_text(gpt: &ChatGPT, text: String, description: String) -> Option<String> {
    println!("> edit_text: {}", colorize_logs(&description));
    let messages = vec![
        Message::new(String::from("system"), None, String::from("you need changes text according to user request and return the result in a format:\n```\ntext result\n```")),
        Message::new(String::from("user"), None, format!("{description}:\n{text}")),
    ];

    match gpt.send(messages, None) {
        Ok(res) => {
            let res = res.expect("Failed to get response");
            if let (Some(start), Some(end)) = (res.find("```"), res.rfind("```")) {
                let res = res[start..end].to_string();
                if let Some(start) = res.find("\n") {
                    return Some(res[start..end].to_string());
                }
            }
            panic!("Failed to get response");
        },
        Err(err) => {
            println!("{}", colorize_logs(&err.to_string()));
            panic!("Failed to get response");
        },
    }
}

pub fn list_dirs(path: String) -> Vec<String> {
    println!("> list_dirs: {}", colorize_logs(&path));
    std::fs::read_dir(path)
        .expect("Failed to read directory")
        .collect::<Vec<_>>()
        .iter()
        .map(|path| path.as_ref().unwrap().path().display().to_string())
        .collect::<Vec<_>>()
}

pub fn pai_run(gpt: &ChatGPT, task: String, flags: Vec<String>) {
    let mut messages = vec![
        Message::new(String::from("system"), None, format!("you are an package manager assistant, your task is to help the user using only these functions: {}\ndon't invent your own functions!", FUNCTIONS.get_names().join(", "))),
        Message::new(String::from("system"), None, format!("user os info: {} {}", std::env::consts::OS, std::env::consts::ARCH)),
        Message::new(String::from("system"), None, format!("user current directory: {}", std::env::current_dir().unwrap().display())),
    ];

    if flags.contains(&String::from("-d")) {
        let path = std::env::current_dir().unwrap().to_str().unwrap().to_string();
        let list = list_dirs(path).join(", ");
        messages.push(Message::new(String::from("system"), None, format!("user directories and files: {list}")));
    }

    messages.push(Message::new(String::from("user"), None, task));

    match gpt.send(messages, Some(FUNCTIONS.to_owned())) {
        Ok(Some(res)) => {
            println!("{}", colorize_logs(&res));
        },
        Err(_) => {
            panic!("Failed to get response from ChatGPT");
        },
        _ => {},
    }
}