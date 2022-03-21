use std::path::PathBuf;
use std::fs;

#[derive(Clone)]
struct Entry {
    path: PathBuf,
    level: usize,
    depth: Vec<bool>,
}

impl Entry {
    fn new(path: PathBuf) -> Self {
        Self { path, level: 0, depth: vec![] }
    }
    fn join(&mut self, file: &str) {
        self.path.push(file);
        self.level += 1;
        self.depth.push(true);
    }
    fn pop(&mut self) {
        self.path.pop();
        self.level -= 1;
        self.depth.pop();
    }
}

fn main() {
    let entry = Entry::new(PathBuf::from("."));
    walk(entry);
}

fn walk(mut entry: Entry) -> Entry {
    if entry.level > 3 {
        return entry;
    }
    let files = fs::read_dir(&entry.path)
        .unwrap()
        .map(|file| file.unwrap().path())
        .collect::<Vec<PathBuf>>();
    for (i, file) in files.iter().enumerate() {
        let file_path = file.as_path();
        let filename = file_path.file_name().unwrap().to_str().unwrap();
        let mut indent = vec![];
        for level in &entry.depth {
            match level {
                true => indent.push(" │ "),
                false => indent.push("   "),
            }
        }
        let symbol = match i {
            i if i + 1 == files.len() => " └ ",
            _ => " ├ "
        };
        let suffix = match file_path.is_dir() {
            true => "/",
            false => "",
        };
        println!("{}{}{}{}", indent.join(""), symbol, filename, suffix);
        if file_path.is_dir() {
            entry.join(filename);
            if i + 1 == files.len() {
                entry.depth[entry.level - 1] = false;
            }
            entry = walk(entry);
            entry.pop();
        }
    }
    entry
}

