use utils::{PAI_GPT_MODEL, PAI_VERSION};


#[macro_use]
extern crate lazy_static;

pub mod chatgpt;
pub mod terminal;
pub mod pai;
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
            println!("pai version: {PAI_VERSION}");
            println!("gpt model: {PAI_GPT_MODEL}");
            return;
        },
        Some("--help") | Some("-h") => {
            println!(r#"
pai [-flags] or [command]

[--key, -k] - set openai key
pai --key 1234567890qwertyuiopasdfghjklzxcvbnm

[--version, -v] - view pai version
pai --version

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
        println!("Use 'pai -k <API-KEY>' to set the key");
        return;
    }

    if task.is_empty() {
        terminal::input_and_processing(flags);
    } else {
        pai::run_task(task);
    }
}
