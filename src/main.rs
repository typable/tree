use std::env;
use std::fs;
use std::io::Result;
use std::path::PathBuf;

#[cfg(windows)]
const LINE_BREAK: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_BREAK: &'static str = "\n";

fn main() {
    let mut path = vec!["."];
    let depth = vec![];
    let mut location = ".";
    let mut file = "";
    let mut all = false;
    let mut level = usize::MAX;
    let mut args = env::args().collect::<Vec<String>>();
    args.drain(0..1);
    for arg in &args {
        match arg {
            arg if arg.starts_with("-p") || arg.starts_with("--path") => {
                match property(arg) {
                    Some(value) => location = value,
                    None => {
                        println!("Error: Invalid argument! Use: [--path|-p]=<location>");
                        return;
                    },
                }
            },
            arg if arg.starts_with("-l") || arg.starts_with("--level") => {
                match property(arg) {
                    Some(value) => {
                        match value.to_string().parse::<usize>() {
                            Ok(depth) => level = depth,
                            Err(_) => {
                                println!("Error: Invalid argument! Use: [--level|-l]=<depth>");
                                return;
                            },
                        }
                    },
                    None => {
                        println!("Error: Invalid argument! Use: [--level|-l]=<depth>");
                        return;
                    },
                }
            },
            arg if arg.starts_with("-o") || arg.starts_with("--out") => {
                match property(arg) {
                    Some(value) => file = value,
                    None => {
                        println!("Error: Invalid argument! Use: [--out|-o]=<file>");
                        return;
                    },
                }
            },
            arg if arg.starts_with("-a") || arg.starts_with("--all") => {
                all = true;
            },
            _ => {
                println!("Error: Unknown argument '{}'!", arg);
                return;
            },
        }
    }
    if location.ne(".") {
        path = location
            .split("/")
            .enumerate()
            .filter(|&(i, part)| i == 0 ||!part.is_empty())
            .map(|(_, part)| part)
            .collect::<Vec<&str>>();
    }
    let mut tree = vec![format!("{}/", path.join("/"))];
    match walk(path, depth, level, all) {
        Ok(result) => {
            let (_, mut sub_tree) = result;
            tree.append(&mut sub_tree);
            let output = tree.join(LINE_BREAK);
            match file.len() > 0 {
                true => {
                    match fs::write(file, output) {
                        Ok(_) => println!("Written into output file."),
                        Err(_) => println!("Unable to resolve location of output file!"),
                    }
                },
                _ => println!("{}", output),
            }
        },
        Err(_) => println!("Error: Unable to resolve location!"),
    }
}

fn walk(path: Vec<&str>, mut depth: Vec<bool>, level: usize, all: bool) -> Result<(Vec<bool>, Vec<String>)> {
    let mut result = Vec::new();
    let tree = indent(&depth);
    let mut alive = true;
    let entries = fs::read_dir(path.join("/"))?
        .map(|entry| entry.unwrap().path())
        .collect::<Vec<PathBuf>>();
    for (i, entry) in entries.iter().enumerate() {
        let symbol = match i {
            i if i + 1 == entries.len() => {
                alive = false;
                " └ "
            },
            i if i < entries.len() => " ├ ",
            _ => " │ ",
        };
        let file = filename(entry);
        if all || !file.starts_with(".") {
            let meta = fs::metadata(format!("{}/{}", path.join("/"), file))?;
            if meta.is_dir() {
                result.push(format!("{}{}{}/", tree, symbol, file));
                if depth.len() < level {
                    let mut branch = path.clone();
                    branch.push(&file);
                    depth.push(alive);
                    let (sub_depth, mut sub_tree) = walk(branch, depth, level, all)?;
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

fn property(arg: &str) -> Option<&str> {
    let parts = arg.split("=")
        .filter(|&part| !part.is_empty())
        .collect::<Vec<&str>>();
    match parts.len() == 2 {
        true => Some(parts[1]),
        _ => None
    }
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
