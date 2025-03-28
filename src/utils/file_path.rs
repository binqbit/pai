use std::{fs, io::{Result, Error, ErrorKind}, path::PathBuf, str::FromStr};

pub struct FilePath {
    path: PathBuf,
}

impl FilePath {
    pub fn new(path: &str) -> Result<FilePath> {
        Ok(FilePath {
            path: PathBuf::from_str(path)
                .map_err(|e|
                    Error::new(ErrorKind::InvalidInput, e)
                )?,
        })
    }

    pub fn join(&self, path: &str) -> Self {
        Self {
            path: self.path.join(path),
        }
    }

    pub fn write_file(&self, content: &str) -> Result<()> {
        let parent = self.path.parent().ok_or_else(|| {
            Error::new(ErrorKind::NotFound, "Parent directory not found")
        })?;
        fs::create_dir_all(parent)?;
        fs::write(&self.path, content)
    }

    pub fn read_file(&self) -> Result<String> {
        fs::read_to_string(&self.path)
    }

    pub fn read_dir(&self) -> Result<Vec<String>> {
        fs::read_dir(&self.path).map(|dirs| {
            dirs.map(|dir| dir
                .unwrap()
                .file_name()
                .into_string()
                .unwrap()
            ).collect()
        })
    }

    pub fn remove_file(&self) -> Result<()> {
        fs::remove_file(&self.path)
    }

    pub fn create_dir(&self) -> Result<()> {
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