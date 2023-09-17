
#[macro_use]
extern crate lazy_static;

pub mod chatgpt;
pub mod pai;
pub mod variables;

pub use chatgpt::*;
pub use pai::*;
pub use variables::*;

fn main() {
    if let Some(mut config) = Config::load() {
        let mut args = std::env::args();
        args.next();
    
        let mut flags = vec![];
        let mut cmd = String::new();
        let mut is_flags = true;
    
        while let Some(arg) = args.next() {
            if arg == "--key"  && is_flags {
                let key = args.next().expect("Missing openai key");
                config.set_openai_key(key);
            } else if arg == "--model"  && is_flags {
                let model = args.next().expect("Missing gpt model");
                config.set_gpt_model(model);
            } else if (arg == "--version" || arg == "-v") && is_flags {
                println!("version: {}", PAI_VERSION);
                println!("gpt model: {}", config.gpt_model);
                return;
            } else if (arg == "--help" || arg == "-h") && is_flags {
                println!(
"pai [-flags] task

[--key] - set openai key
pai --key 1234567890qwertyuiopasdfghjklzxcvbnm

[--model] - set gpt model
pai --model gpt-4

[--version, -v] - view pai version
pai --version

[--help, -h] - view help

[-d] - show show directory folders");
                return;
            } else if arg.starts_with('-') && is_flags {
                flags.push(arg);
            } else {
                is_flags = false;
                cmd = format!("{} {}", cmd, arg);
            }
        }

        config.save();
    
        if let Some(gpt) = ChatGPT::new(config) {
            pai_run(&gpt, cmd, flags);
        } else {
            println!("Missing openai key");
        }
    }
}
