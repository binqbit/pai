use chatgpt::ChatGPT;
use utils::{print_markdown, GPT_MODEL};

#[macro_use]
extern crate lazy_static;

pub mod chatgpt;
pub mod terminal;
pub mod utils;

fn main() {
    let mut args = std::env::args();
    args.next();

    let mut flags = vec![];
    let mut task = String::new();

    match args.next().as_deref() {
        Some("--key") | Some("-k") => {
            let key = args.next().expect("Missing OpenAI API-Key!");
            chatgpt::set_apikey(&key);
            return;
        },
        Some("--version") | Some("-v") => {
            println!("sai version: {}", env!("CARGO_PKG_VERSION"));
            println!("gpt model: {}", GPT_MODEL);
            return;
        },
        Some("--help") | Some("-h") => {
            println!(r#"
sai [-flags] or [command]

[--key, -k] - set openai key
sai --key 1234567890qwertyuiopasdfghjklzxcvbnm

[--version, -v] - view sai version
sai --version

[--help, -h] - view help
"#);
            return;
        },
        Some(arg) => {
            if arg.starts_with("-") {
                flags.push(arg.to_string());
            } else {
                task = arg.to_string();
            }
            while let Some(arg) = args.next() {
                if task.is_empty() && arg.starts_with("-") {
                    flags.push(arg);
                } else {
                    task = format!("{} {}", task, arg);
                }
            }
        },
        _ => {},
    }

    if chatgpt::get_apikey().is_none() {
        println!("OpenAI API-Key not set!");
        println!("Use 'sai -k <API-KEY>' to set the key");
        return;
    }

    if task.is_empty() {
        terminal::input_and_processing(flags);
    } else {
        match ChatGPT::for_assistant().run_assistant(task) {
            Ok(text) => {
                print_markdown(&text);
            },
            Err(err) => {
                eprintln!("Error: {err:?}");
            },
        }
    }
}
