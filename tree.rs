use std::env;
use std::fs;
use std::io::Result;
use std::path::PathBuf;

fn main() {
    let mut path = vec!["."];
    let depth = vec![];
    let mut location = ".";
    let mut all = false;
    let mut level = usize::MAX;
    let mut args = env::args().collect::<Vec<String>>();
    args.drain(0..1);
    for arg in &args {
        match arg {
            arg if arg.starts_with("-p") || arg.starts_with("--path") => {
                let parts = arg.split("=")
                    .filter(|&part| !part.is_empty())
                    .collect::<Vec<&str>>();
                if parts.len() == 2 {
                    location = parts[1];
                }
                else {
                    println!("Error: Invalid argument! Use: [--path|-p]=<location>");
                    return;
                }
            }
            arg if arg.starts_with("-l") || arg.starts_with("--level") => {
                let parts = arg.split("=")
                    .filter(|&part| !part.is_empty())
                    .collect::<Vec<&str>>();
                if parts.len() == 2 {
                    match parts[1].to_string().parse::<usize>() {
                        Ok(depth) => level = depth,
                        Err(_) => {
                            println!("Error: Invalid argument! Use: [--level|-l]=<depth>");
                            return;
                        },
                    }
                }
                else {
                    println!("Error: Invalid argument! Use: [--level|-l]=<depth>");
                    return;
                }
            }
            arg if arg.starts_with("-a") || arg.starts_with("--all") => {
                all = true;
            }
            _ => {
                println!("Error: Unknown argument '{}'!", arg);
                return;
            },
        }
    }
    if location.ne(".") {
        path = location
            .split("/")
            .filter(|&part| !part.is_empty())
            .collect::<Vec<&str>>();
    }
    println!("{}/", path.join("/"));
    match walk(path, depth, level, all) {
        Ok(_) => (),
        Err(_) => println!("Error: Unable to resolve location!"),
    }
}

fn walk(path: Vec<&str>, mut depth: Vec<bool>, level: usize, all: bool) -> Result<Vec<bool>> {
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
                println!("{}{}{}/", tree, symbol, file);
                if depth.len() < level {
                    let mut branch = path.clone();
                    branch.push(&file);
                    depth.push(alive);
                    depth = walk(branch, depth, level, all)?;
                }
            }
            else {
                println!("{}{}{}", tree, symbol, file);
            }
        }
    }
    depth.pop();
    Ok(depth)
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
