use std::fs;
use std::io::Result;
use std::path::PathBuf;

use tree::Options;

#[cfg(windows)]
const NEWLINE: &'static str = "\r\n";
#[cfg(not(windows))]
const NEWLINE: &'static str = "\n";

fn main() {
    let options = Options::from_args();
    let mut tree = vec![options.path.to_string_lossy().to_string() + "/"];
    match walk(options.path, vec![], options.level_limit, options.all_files) {
        Ok(result) => {
            let (_, mut sub_tree) = result;
            tree.append(&mut sub_tree);
            println!("{}", tree.join(NEWLINE));
        },
        Err(err) => eprintln!("{:#?}", err),
    }
}

fn walk(
    path: PathBuf,
    mut depth: Vec<bool>,
    level: usize,
    all: bool,
) -> Result<(Vec<bool>, Vec<String>)> {
    let mut result = Vec::new();
    let tree = indent(&depth);
    let mut alive = true;
    let entries = fs::read_dir(&path)?
        .map(|entry| entry.unwrap().path())
        .collect::<Vec<PathBuf>>();
    for (i, entry) in entries.iter().enumerate() {
        let symbol = match i {
            i if i + 1 == entries.len() => {
                alive = false;
                " └ "
            }
            i if i < entries.len() => " ├ ",
            _ => " │ ",
        };
        let file = filename(&entry);
        if all || !file.starts_with(".") {
            let meta = fs::metadata(&entry)?;
            if meta.is_dir() {
                result.push(format!("{}{}{}/", tree, symbol, file));
                if depth.len() < level {
                    depth.push(alive);
                    let (sub_depth, mut sub_tree) = walk(entry.clone(), depth, level, all)?;
                    depth = sub_depth;
                    result.append(&mut sub_tree);
                }
            }
            else {
                result.push(format!("{}{}{}", tree, symbol, file));
            }
        }
    }
    depth.pop();
    Ok((depth, result))
}

fn filename(entry: &PathBuf) -> String {
    String::from(entry.file_name().unwrap().to_str().unwrap())
}

fn indent(depth: &Vec<bool>) -> String {
    let mut list = Vec::new();
    for level in depth {
        match level {
            true => list.push(" │ "),
            false => list.push("   "),
        }
    }
    list.join("")
}
