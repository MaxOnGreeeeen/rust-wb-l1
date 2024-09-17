use std::io;

fn main() -> Result<(), io::Error> {
    let mut buffer = String::new();

    println!("Введите строку: ");

    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;

    let string_to_reverse = buffer.trim();
    let reversed_string = reverse_string(string_to_reverse);

    println!("{}", reversed_string);

    Ok(())
}

// Разворачивает строку
pub fn reverse_string(string_to_reverse: &str) -> String {
    string_to_reverse.chars().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    use crate::reverse_string;

    #[test]
    fn reverse_string_test_unicode() {
        let string_to_reverse = "123✅456#$";

        assert_eq!(reverse_string(string_to_reverse), "$#654✅321");
    }

    #[test]
    fn reverse_string_test_chars() {
        let string_to_reverse = "abcde";

        assert_eq!(reverse_string(string_to_reverse), "edcba");
    }
}
