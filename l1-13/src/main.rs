use std::io;

fn main() -> Result<(), io::Error> {
    let mut buffer = String::new();

    println!("Введите последовательность строк: ");

    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;

    let buf_strings = buffer.trim();

    let unique_strings = unique(buf_strings);

    println!("{:?}", unique_strings);

    Ok(())
}

// Удаление дубликатов из массива строк
pub fn unique(lines: &str) -> String {
    // Строка для хранения уникальных строк
    let mut unique_lines = String::new();

    // Переменная для хранеиия последней добавленной строки
    let mut last = None;

    // Проходим по отсортированным строкам и добавляем уникальные
    for line in lines.split_whitespace() {
        if last.as_ref() != Some(&line) {
            let line_to_add = &(line.to_owned() + " ");

            unique_lines.push_str(line_to_add);
            last = Some(line);
        }
    }

    unique_lines.trim().to_owned()
}

#[cfg(test)]
mod tests {
    use crate::unique;

    #[test]
    fn remove_duplicates_test() {
        let test_str_vec = "dsdfsdf sdf sdf sdf s";
        let remove_duplicates_result = unique(test_str_vec);
        assert_eq!(remove_duplicates_result, "dsdfsdf sdf s");
    }
}
