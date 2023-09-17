
use regex::{Regex, Captures};

lazy_static! {
    static ref COMMAND_PATTERN: Regex = Regex::new(r"(\b\d+\b)|(^(\./)?\w[\w\-\.]*\b)|(\-{1,2}\w[\w\-\.]*\b)|(\b\w[\w\-\.]*\b)").unwrap();
    static ref LOGS_PATTERN: Regex = Regex::new(r"(\W+)|(\b\d+\b)").unwrap();
}

const CMD_COMMAND_COLOR: &str = "\x1b[33m";
const CMD_PARAMETER_COLOR: &str = "\x1b[37m";
const CMD_FLAGS_COLOR: &str = "\x1b[90m";
const CMD_NUMBER_COLOR: &str = "\x1b[34m";
const CMD_DEFAULT_COLOR: &str = "\x1b[0m";

const LOGS_NUMBER_COLOR: &str = "\x1b[34m";
const LOGS_SYMBOL_COLOR: &str = "\x1b[32m";
const LOGS_DEFAULT_COLOR: &str = "\x1b[0m";

fn get_command_color(caps: &Captures) -> &'static str {
    if caps.get(1).is_some() {
        return CMD_NUMBER_COLOR;
    } else if caps.get(2).is_some() {
        return CMD_COMMAND_COLOR;
    } else if caps.get(6).is_some() {
        return CMD_COMMAND_COLOR;
    } else if caps.get(7).is_some() {
        return CMD_FLAGS_COLOR;
    } else if caps.get(8).is_some() {
        return CMD_FLAGS_COLOR;
    } else if caps.get(9).is_some() {
        return CMD_PARAMETER_COLOR;
    }

    CMD_DEFAULT_COLOR
}

fn get_logs_color(caps: &Captures) -> &'static str {
    if caps.get(1).is_some() {
        return LOGS_SYMBOL_COLOR;
    }
    if caps.get(2).is_some() {
        return LOGS_NUMBER_COLOR;
    }
    LOGS_DEFAULT_COLOR
}

pub fn colorize_command(message: &str) -> String {
    COMMAND_PATTERN.replace_all(message, |caps: &Captures| -> String {
        format!("{}{}{}", get_command_color(caps), caps.get(0).unwrap().as_str(), CMD_DEFAULT_COLOR)
    }).to_string()
}

pub fn colorize_logs(message: &str) -> String {
    LOGS_PATTERN.replace_all(message, |caps: &Captures| -> String {
        format!("{}{}{}", get_logs_color(caps), caps.get(0).unwrap().as_str(), LOGS_DEFAULT_COLOR)
    }).to_string()
}