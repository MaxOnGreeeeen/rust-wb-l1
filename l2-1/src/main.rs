use regex::Regex;
use std::{env, fs, io, path::Path};

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

fn main() -> io::Result<()> {
    let reg_exp = Regex::new(r"wc (-[c|l|w] ){0,1}([a-zA-Z0-9_.+-|\|//]+\.txt){1}").unwrap();
    for line in io::stdin().lines() {
        let line_unwraped = line?;

        let Some(caps) = reg_exp.captures(&line_unwraped) else {
            println!("Incorrect command");
            continue;
        };

        let file_path = caps.get(2).map_or("", |m| m.as_str());
        let file_read_mode = caps.get(1).map_or("", |m| m.as_str()).trim();

        let file_calculation_option = Options::from(file_read_mode);
        let result = match process_file_calculations(&file_path, file_calculation_option) {
            Ok(res_string) => res_string,
            Err(_) => format!("Error reading file with specified path"),
        };

        println!("{}", result);
    }

    Ok(())
}

fn process_file_calculations(file_path: &str, mode: Options) -> io::Result<String> {
    let relative_file_path = match Path::new(file_path).is_relative() {
        true => &Path::new(&env::current_dir()?).join(&Path::new(file_path)),
        _ => Path::new(file_path),
    };

    let contents = fs::read_to_string(relative_file_path)?;
    return match mode {
        Options::Strings => {
            let result_calc = contents
                .chars()
                .filter(|&symbol| symbol.to_string() == "\n")
                .count();
            Ok(format!(
                "Strings amount in file {file_path} is: {result_calc}"
            ))
        }
        Options::Symbols => {
            let symbols_amount = contents.len();
            Ok(format!(
                "Symbols amount in file {file_path} is: {symbols_amount}"
            ))
        }
        Options::Words => {
            let words_amount = contents.split(" ").collect::<Vec<&str>>().len();
            Ok(format!(
                "Words amount in file {file_path} is: {words_amount}"
            ))
        }
    };
}
