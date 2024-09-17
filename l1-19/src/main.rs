fn main() {
    let test_str = "snow dog sun";
    println!("{}", reverse_words(test_str));
}

// Возвращает строку с реверсивном порядка слов
pub fn reverse_words(source_string: &str) -> String {
    source_string
        .split(" ")
        .collect::<Vec<&str>>()
        .into_iter()
        .rev()
        .collect::<Vec<&str>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use crate::reverse_words;

    #[test]
    fn reverse_words_test() {
        let test_str = "snow dog sun";
        assert_eq!(reverse_words(test_str), "sun dog snow");
    }
}
