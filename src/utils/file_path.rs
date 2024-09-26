use std::{fs, path::PathBuf, str::FromStr};

pub struct FilePath {
    path: PathBuf,
}

impl FilePath {
    pub fn new(path: &str) -> std::io::Result<FilePath> {
        Ok(FilePath {
            path: PathBuf::from_str(path).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?,
        })
    }

    pub fn join(&self, path: &str) -> Self {
        Self {
            path: self.path.join(path),
        }
    }

    pub fn write_file(&self, content: &str) -> std::io::Result<()> {
        fs::write(&self.path, content)
    }

    pub fn read_file(&self) -> std::io::Result<String> {
        fs::read_to_string(&self.path)
    }

    pub fn read_dir(&self) -> std::io::Result<Vec<String>> {
        fs::read_dir(&self.path).map(|dirs| {
            dirs.map(|dir| {
                dir.unwrap().file_name().into_string().unwrap()
            }).collect()
        })
    }

    pub fn remove_file(&self) -> std::io::Result<()> {
        fs::remove_file(&self.path)
    }

    pub fn create_dir(&self) -> std::io::Result<()> {
        fs::create_dir(&self.path)
    }

    pub fn is_exists(&self) -> bool {
        self.path.exists()
    }

    pub fn is_dir(&self) -> bool {
        self.path.is_dir()
    }

    pub fn get_path(&self) -> String {
        self.path.to_str().expect("Failed to get path").to_owned()
    }
}