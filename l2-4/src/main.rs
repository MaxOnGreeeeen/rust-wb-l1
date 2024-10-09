use std::collections::HashMap;

use once_cell::sync::Lazy;

fn main() {}

static ALPHABET: &str = "абвгдеёжзийклмнопрстуфхцчшщъыьэюя";
static ALPHABET_INDECES: Lazy<HashMap<String, usize>> = Lazy::new(|| {
    HashMap::from_iter(ALPHABET.chars().enumerate().map(|(index, char)| {
        return (char.to_string(), index);
    }))
});

// Генерация ключа мапы
fn generate_word_key(word: &str) -> [u32; 33] {
    let mut array_indexes = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0,
    ];
    word.chars().for_each(|char| {
        let char_alphabet_index = ALPHABET_INDECES.get(&char.to_string()).unwrap();
        array_indexes[*char_alphabet_index] += 1;
    });

    return array_indexes;
}

pub fn search_anagrams<'a>(strings_array: &'a [&'a str]) -> HashMap<String, Vec<String>> {
    let mut result_hash_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut temp_values_hash_map: HashMap<[u32; 33], Vec<String>> = HashMap::new();

    for &word in strings_array {
        let word_lower = word.to_lowercase();
        let word_key = generate_word_key(&word_lower);
        temp_values_hash_map
            .entry(word_key)
            .and_modify(|words| {
                if !words.contains(&word_lower) {
                    words.push(word_lower.clone());
                }
            })
            .or_insert(vec![word_lower]);
    }

    for (_, value) in temp_values_hash_map.iter_mut() {
        if value.len() > 1 {
            value.sort();
            let first_word = value[0].clone();
            result_hash_map.insert(first_word, value.clone());
        }
    }

    result_hash_map
}

#[cfg(test)]
mod tests {
    use super::*;

    // Поиск анаграм и их группировка
    #[test]
    fn unzip_simple_string() {
        let words = vec![
            "пятак",
            "пятка",
            "тяпка",
            "листок",
            "слиток",
            "столик",
            "пятак",
        ];

        let mut expected_anagrams: HashMap<&str, Vec<&str>> = HashMap::new();
        expected_anagrams.insert("пятак", vec!["пятак", "пятка", "тяпка"]);
        expected_anagrams.insert("листок", vec!["листок", "слиток", "столик"]);

        let result = search_anagrams(&words);

        assert_eq!(result.len(), expected_anagrams.len());
        for (key, group) in result {
            assert_eq!(group, *expected_anagrams.get(&key as &str).unwrap());
        }
    }
}
