use {once_cell::sync::Lazy, regex::Regex};

fn main() {}

static REG_EX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\\.|[a-zA-Z])(\d*)").unwrap());
static VALIDATE_REG_EX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^((\\.|[a-zA-Z])(\d*))*$").unwrap());

// Функция распаковки строки
pub fn unzip_string(zipped_string: &str) -> Result<String, String> {
    if !is_valid_string(zipped_string) {
        return Err("Incorrect String".to_string());
    }

    let mut result = String::new();
    for cap in REG_EX.captures_iter(zipped_string) {
        let part = &cap[1];
        let count_str = &cap[2];
        let repeat_count = match count_str.is_empty() {
            true => 1,
            false => count_str.parse::<usize>().unwrap_or(1),
        };

        if part.starts_with('\\') {
            let escaped_char = part.chars().nth(1).unwrap();
            for _ in 0..repeat_count {
                result.push(escaped_char);
            }
        } else {
            for _ in 0..repeat_count {
                result.push(part.chars().nth(0).unwrap());
            }
        }
    }
    Ok(result)
}

fn is_valid_string(validated_string: &str) -> bool {
    if validated_string.starts_with(char::is_numeric) {
        return false;
    }
    if REG_EX.captures(validated_string).is_none() && validated_string.len() != 0 {
        return false;
    }
    VALIDATE_REG_EX.is_match(validated_string)
}

#[cfg(test)]
mod tests {
    use crate::unzip_string;

    // Распаковка строки включающей все возможные случаи распаковки, включая escape-символы
    #[test]
    fn unzip_simple_string() {
        let string = r"q3w1e\54\\4";
        assert_eq!(unzip_string(string).unwrap(), r"qqqwe5555\\\\");
    }

    // Распаковка пустой строки завершается успешно
    #[test]
    fn unzip_empty_string() {
        let string = "";
        assert_eq!(unzip_string(string).unwrap(), "");
    }

    // Распаковка, завершающаяся с ошибкой (начинается со строки)
    #[test]
    fn unzip_string_error() {
        let string = "45";
        assert_eq!(
            unzip_string(string).map_err(|e| e),
            Err("Incorrect String".to_string())
        );
    }

    // Распаковка, завершающаяся ошибкой из-за незавершенной escape-последовательности
    #[test]
    fn unzip_string_error_escape() {
        let string = r"a\\\";
        assert_eq!(
            unzip_string(string).map_err(|e| e),
            Err("Incorrect String".to_string())
        );
    }
}
