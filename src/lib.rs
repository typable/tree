use std::env;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Options {
    pub path: PathBuf,
    pub level_limit: usize,
    pub all_files: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            path: PathBuf::from("."),
            level_limit: usize::MAX,
            all_files: false,
        }
    }
}

impl Options {
    pub fn from_args() -> Self {
        let mut options = Self::default();
        let mut args = env::args().skip(1);
        let mut target = None;
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--path" => target = Some("path"),
                "--level" => target = Some("level"),
                "--all" => options.all_files = true,
                "-p" => target = Some("path"),
                "-l" => target = Some("level"),
                "-a" => options.all_files = true,
                _ => {
                    if target == None {
                        unreachable!();
                    }
                    match target.unwrap() {
                        "path" => options.path = PathBuf::from(arg),
                        "level" => options.level_limit = arg.parse().unwrap(),
                        _ => unreachable!(),
                    }
                    target = None;
                },
            }
        }
        options
    }
}
