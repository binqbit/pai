use crate::get_exec_path;



pub fn read_database() -> String {
    let path = get_exec_path().join("database");
    let files = std::fs::read_dir(path)
        .expect("Failed to read directory");
    let mut db = String::new();
    for file in files {
        let content = std::fs::read_to_string(file.unwrap().path())
            .expect("Something went wrong reading the file");
        db = format!("{}\n{}", db, content);
    }
    db
}