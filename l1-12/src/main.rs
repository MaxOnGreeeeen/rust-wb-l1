use std::collections::HashSet;

// Возвращает пересечение двух множеств
// (Но можно было бы использовать стандартный HashSet::intersection)
fn intersection<T: Eq + std::hash::Hash + Clone>(
    set1: &HashSet<T>,
    set2: &HashSet<T>,
) -> HashSet<T> {
    let (smaller, larger) = if set1.len() < set2.len() {
        (set1, set2)
    } else {
        (set2, set1)
    };

    let mut result = HashSet::new();
    for elem in smaller {
        if larger.contains(elem) {
            result.insert(elem.clone());
        }
    }
    result
}

fn main() {
    let set1: HashSet<i32> = [1, 2, 3, 4, 5].iter().cloned().collect();
    let set2: HashSet<i32> = [3, 4, 5, 6, 7].iter().cloned().collect();

    let result = intersection(&set1, &set2);

    println!("Пересечение: {:?}", result);

    let mut result_vec = result.into_iter().collect::<Vec<i32>>();
    result_vec.sort();

    let result_slice = &result_vec[..];

    assert_eq!(result_slice, &[3, 4, 5] as &[i32]);
}
