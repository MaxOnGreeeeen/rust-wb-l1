use regex::Regex;
use std::{env, fs, io, path::Path};
use {once_cell::sync::Lazy, regex::Regex};

#[derive(Debug)]
enum Options {
    Symbols,
    Strings,
    Words,
}
impl From<&str> for Options {
    fn from(value: &str) -> Self {
        return match value {
            "-c" => Options::Symbols,
            "-l" => Options::Strings,
            "-w" => Options::Words,
            _ => Options::Words,
        };
    }
}

static REG_EX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"sort (-[c|l|w]){0,1}([a-zA-Z0-9_.+-|\|//]+\.txt){1}").unwrap());

fn main() -> io::Result<()> {
    for line in io::stdin().lines() {}

    Ok(())
}

fn sort_file_strings(file_path: &str, mode: Options) -> io::Result<String> {
    let relative_file_path = match Path::new(file_path).is_relative() {
        true => &Path::new(&env::current_dir()?).join(&Path::new(file_path)),
        _ => Path::new(file_path),
    };
    let contents = fs::read_to_string(relative_file_path)?;

    Ok(format!("Sorted strings of file {file_path}: \n"))
}
