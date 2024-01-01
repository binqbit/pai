

pub fn list_files(path: &str) -> String {
    let mut dirs = String::new();
    for entry in std::fs::read_dir(path).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        if path.is_dir() {
            dirs.push_str(&format!("{}/\n", entry.file_name().to_str().unwrap()));
        } else {
            dirs.push_str(&format!("{}\n", entry.file_name().to_str().unwrap()));
        }
    }
    dirs
}