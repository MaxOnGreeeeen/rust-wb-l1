use std::io;

fn main() -> Result<(), io::Error> {
    let mut buffer = String::new();

    println!("Введите последовательность строк: ");

    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;

    let buf_strings = buffer.trim().split(" ").collect::<Vec<&str>>();

    let unique_strings = remove_duplicates(buf_strings);

    println!("{:?}", unique_strings);

    Ok(())
}

// Удаление дубликатов из массива строк
pub fn remove_duplicates(mut lines: Vec<&str>) -> Vec<&str> {
    lines.sort();

    // Вектор для хранения уникальных строк
    let mut unique_lines = vec![];

    // Переменная для хранеиия последней добавленной строки
    let mut last = None;

    // Проходим по отсортированным строкам и добавляем уникальные
    for line in lines {
        if last.as_ref() != Some(&line) {
            unique_lines.push(line);
            last = Some(line);
        }
    }

    unique_lines
}

#[cfg(test)]
mod tests {
    use crate::remove_duplicates;

    #[test]
    fn remove_duplicates_test() {
        let test_str_vec = vec!["dsdfsdf", "sdf", "sdf", "sdf", "s"];
        let remove_duplicates_result = remove_duplicates(test_str_vec);
        assert_eq!(remove_duplicates_result, vec!["dsdfsdf", "s", "sdf"]);
    }
}
