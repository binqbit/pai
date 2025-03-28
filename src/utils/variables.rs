use std::{env, path::PathBuf};

use crate::chatgpt::get_apikey;

use super::FilePath;

pub const GPT_MODEL: &str = "gpt-4o";

lazy_static! {
    pub static ref EXE_PATH: PathBuf = env::current_exe()
        .unwrap().parent()
        .unwrap().parent()
        .unwrap().to_owned();
    pub static ref CUR_PATH: PathBuf = env::current_dir()
        .unwrap().to_owned();
    pub static ref API_KEY: String = get_apikey()
        .expect("API key not found");
}

pub fn get_exec_path<'a>() -> &'a PathBuf {
    &EXE_PATH
}

pub fn get_current_path<'a>() -> &'a PathBuf {
    &CUR_PATH
}

impl FilePath {
    pub fn exec() -> FilePath {
        FilePath::new(
            get_exec_path()
                .to_str().unwrap()
        ).unwrap()
    }

    pub fn current() -> FilePath {
        FilePath::new(
            get_current_path()
                .to_str().unwrap()
        ).unwrap()
    }
}