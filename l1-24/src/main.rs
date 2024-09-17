use std::collections::HashSet;

fn main() {
    assert_eq!(unique("abCdefAaf"), false);
    assert_eq!(unique("abcd"), true);
}

fn unique(string: &str) -> bool {
    let unique_set: HashSet<char> = string.to_lowercase().chars().collect();
    unique_set.len() == string.len()
}
