
#[macro_use]
extern crate lazy_static;

use dotenv::dotenv;

pub mod utils;
pub mod chatgpt;
pub mod pai;

pub use utils::*;
pub use chatgpt::*;
pub use pai::*;

fn main() {
    dotenv().ok();

    let mut args = std::env::args();
    args.next();

    let mut flags = vec![];
    let mut cmd = String::new();
    let mut is_flags = true;

    for arg in args {
        if arg.starts_with('-') && is_flags {
            flags.push(arg);
        } else {
            is_flags = false;
            cmd = format!("{} {}", cmd, arg);
        }
    }

    let gpt = ChatGPT::new(OPENAI_KEY.to_string());

    pai_run(&gpt, cmd, flags);
}
