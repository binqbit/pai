use super::ProjectDir;

impl ProjectDir {
    fn _tree(&mut self, path: &str, tree_path: &str, fs_tree: &mut String, is_print: bool) -> Result<(), std::io::Error> {
        let entries = self.list_files(path)?;
        let entries_count = entries.len();
        for (index, entry) in entries.into_iter().enumerate() {
            let file_name = entry.file_name().into_string().unwrap();
            let is_last = index == entries_count - 1;
            let is_dir = entry.file_type().unwrap().is_dir();
            let tree_line = format!("{}{}{}{}",
                tree_path,
                if is_last { "└─" } else { "├─" },
                file_name,
                if is_dir { "/" } else { "" });
            fs_tree.push_str(&tree_line);
            fs_tree.push_str("\n");
            if (is_print) {
                println!("{tree_line}");
            }
            let tree_path = if is_last {
                format!("{}{}", tree_path, "  ")
            } else {
                format!("{}{}", tree_path, "│ ")
            };
            if is_dir {
                self._tree(&entry.path().to_str().unwrap(), &tree_path, fs_tree, is_print)?;
            }
        }
        Ok(())
    }

    pub fn tree(&mut self, path: &str, is_print: bool) -> Result<String, std::io::Error> {
        let mut fs_tree = String::new();
        self._tree(path, "", &mut fs_tree, is_print)?;
        Ok(fs_tree)
    }
}