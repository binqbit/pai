use crate::utils::FilePath;

impl FilePath {
    pub fn config() -> FilePath {
        FilePath::exec().join("config")
    }

    pub fn temp() -> FilePath {
        FilePath::exec().join("temp")
    }

    pub fn scripts() -> FilePath {
        FilePath::exec().join("scripts")
    }
}

pub fn get_apikey() -> Option<String> {
    FilePath::config().join("apikey.txt").read_file().ok()
}

pub fn set_apikey(apikey: &str) {
    FilePath::config().join("apikey.txt").write_file(apikey).unwrap();
}
