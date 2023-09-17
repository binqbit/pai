use std::env;

lazy_static! {
    pub static ref CHATGPT_MODEL: String =
        env::var("CHATGPT_MODEL").unwrap_or(String::from("gpt-3.5-turbo"));
    pub static ref OPENAI_KEY: String =
        env::var("OPENAI_KEY").expect("Missing OPENAI_KEY environment variable.");
}