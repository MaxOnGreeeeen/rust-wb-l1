fn main() {
    let mut vec_numbers = vec![1, 4, 7, 2, 6];

    println!("{}", binary_search(&mut vec_numbers, 2));
    println!("{}", binary_search(&mut vec_numbers, 3));
}

pub fn binary_search<T: Ord>(arr: &mut [T], element: T) -> bool {
    arr.sort();

    let mut low = 0;
    let mut high = arr.len() - 1;
    let mut middle;

    while low <= high {
        middle = (low + high) / 2;
        if element < arr[middle] {
            high = middle - 1;
        } else if element > arr[middle] {
            low = middle + 1;
        } else {
            return true;
        }
    }

    return false;
}

#[cfg(test)]
mod tests {
    use crate::binary_search;

    #[test]
    fn binary_search_test_true() {
        let mut vec_numbers = vec![1, 4, 7, 2, 6];

        assert_eq!(binary_search(&mut vec_numbers, 2), true);
    }

    #[test]
    fn binary_search_test_false() {
        let mut vec_numbers = vec![1, 4, 7, 2, 6];

        assert_eq!(binary_search(&mut vec_numbers, 3), false);
    }
}
