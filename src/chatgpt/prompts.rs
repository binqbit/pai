use crate::utils::FilePath;

impl FilePath {
    pub fn prompts() -> FilePath {
        FilePath::config().join("prompts")
    }
}

lazy_static! {
    pub static ref SUGGESTIONS: String = FilePath::prompts().join("suggestions.txt").read_file().expect("Failed to load suggestions prompt file");
    pub static ref COMMANDS: String = FilePath::prompts().join("commands.txt").read_file().expect("Failed to load commands prompt file");
}