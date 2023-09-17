use std::{env, path::{PathBuf, Path}};


pub const PAI_VERSION: &str = "v1.0.2";
pub const PAI_GPT_MODEL: &str = "gpt-3.5-turbo";

lazy_static! {
    pub static ref EXE_PATH: PathBuf = env::current_exe().unwrap();
}

pub fn get_exec_path<'a>() -> &'a Path {
    EXE_PATH.parent().unwrap()
}