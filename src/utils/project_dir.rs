use std::{collections::HashSet, fs::{self, DirEntry}, io::Read, path::Path};



pub struct ProjectDir {
    ignored: HashSet<String>,
}



impl ProjectDir {
    pub fn new() -> Self {
        Self {
            ignored: HashSet::new(),
        }
    }

    pub fn check_ignore_file(&mut self, path: &str) -> Result<(), std::io::Error> {
        let ignore_file = Path::new(path).join(".gitignore");
        if !ignore_file.exists() {
            return Ok(());
        }
        let content = std::fs::read_to_string(ignore_file)?;
        let path = normalize_path(path);
        let ignored = content.lines()
            .filter(|line| !line.starts_with("#") && !line.is_empty())
            .map(|line| format!("{}{}", path, normalize_path(line)));
        self.ignored.extend(ignored);
        Ok(())
    }
    
    pub fn is_ignore_path(&self, path: &str) -> bool {
        let path = normalize_path(path);
        self.ignored.iter().any(|ignore| path.starts_with(ignore)) || path.ends_with("/.git/") || path.ends_with("/.idea/") || path.ends_with("/.vscode/")
    }

    pub fn list_files(&mut self, path: &str) -> Result<Vec<DirEntry>, std::io::Error> {
        self.check_ignore_file(path)?;
        Ok(fs::read_dir(path)?
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| {
                if self.is_ignore_path(entry.path().to_str().unwrap()) {
                    None
                } else {
                    Some(entry)
                }
            })
            .collect::<Vec<_>>())
    }

    pub fn load_text_files(&mut self, path: &str) -> Result<Vec<String>, std::io::Error> {
        Ok(self.list_files(path)?
            .into_iter()
            .filter(|entry| entry.path().is_file())
            .filter(|entry| !is_binary(entry.path().to_str().unwrap()))
            .map(|entry| entry.path().to_str().unwrap().to_owned())
            .collect())
    }

    pub fn list_dirs(&mut self, path: &str) -> Result<Vec<String>, std::io::Error> {
        Ok(self.list_files(path)?
            .into_iter()
            .filter(|entry| entry.path().is_dir())
            .map(|entry| entry.path().to_str().unwrap().to_owned())
            .collect())
    }

    pub fn map_all_text_files(&mut self, path: &str, f: &mut dyn FnMut(String) -> ()) -> Result<(), std::io::Error> {
        for entry in self.list_files(path)? {
            let path = entry.path().to_str().unwrap().to_owned();
            if entry.path().is_dir() {
                self.map_all_text_files(&path, f)?;
            } else {
                if !is_binary(&path) {
                    f(path);
                }
            }
        }
        Ok(())
    }

    pub fn get_all_files(&mut self, path: &str) -> Result<Vec<String>, std::io::Error> {
        let mut files = Vec::new();
        for entry in self.list_files(path)? {
            let path = entry.path().to_str().unwrap().to_owned();
            if entry.path().is_dir() {
                files.extend(self.get_all_files(&path)?);
            } else {
                files.push(path);
            }
        }
        Ok(files)
    }

    pub fn get_all_text_files(&mut self, path: &str) -> Result<Vec<String>, std::io::Error> {
        let mut files = Vec::new();
        for entry in self.list_files(path)? {
            let path = entry.path().to_str().unwrap().to_owned();
            if entry.path().is_dir() {
                files.extend(self.get_all_text_files(&path)?);
            } else {
                if !is_binary(&path) {
                    files.push(path);
                }
            }
        }
        Ok(files)
    }
}



fn normalize_path(path: &str) -> String {
    path.replace("\\", "/")
        .trim_end_matches("/")
        .trim_start_matches("/")
        .to_string() + "/"
}

pub fn is_binary(path: &str) -> bool {
    let mut file = fs::File::open(path).unwrap();
    let mut buffer = [0; 1024];
    let read_bytes = file.read(&mut buffer).unwrap();
    std::str::from_utf8(&buffer[..read_bytes]).is_err()
}