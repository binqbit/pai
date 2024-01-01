
#[macro_use]
extern crate lazy_static;

pub mod pai;
pub mod utils;

pub use pai::*;
pub use utils::*;



fn main() {
    let mut args = std::env::args();
    args.next();

    let mut command = if let Some(arg) = args.next() {
        if arg == "--key" || arg == "-k" {
            let key = args.next().expect("Missing openai key");
            Config::set_apikey(key);
            return;
        } else if arg == "--version" || arg == "-v" {
            println!("version: {PAI_VERSION}");
            println!("gpt model: {PAI_GPT_MODEL}");
            return;
        } else if arg == "--help" || arg == "-h" {
            println!(r#"
pai [-flags] or [command]

[--key, -k] - set openai key
pai --key 1234567890qwertyuiopasdfghjklzxcvbnm

[--version, -v] - view pai version
pai --version

[--help, -h] - view help
"#);
            return;
        } else {
            arg
        }
    } else {
        return;
    };

    while let Some(arg) = args.next() {
        command = format!("{} {}", command, arg);
    }

    let command = command.trim().to_string();

    if let Some(config) = Config::load() {
        let gpt = ChatGPT::new(&config);
        pai_run(&gpt, command);
    } else {
        println!("Missing openai key!");
    }
}
