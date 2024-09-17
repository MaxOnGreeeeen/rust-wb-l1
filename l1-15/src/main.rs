fn main() {
    let mut vec_numbers = vec![2, 1, 4, 5, 3];
    quick_sort(&mut vec_numbers);
    println!("{:?}", vec_numbers);
}

pub fn quick_sort<T: Ord + Clone>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_index = partition(arr);
    let (left, right) = arr.split_at_mut(pivot_index);

    quick_sort(&mut left[..pivot_index]);
    quick_sort(&mut right[1..]);
}

fn partition<T: Ord + Clone>(arr: &mut [T]) -> usize {
    let len = arr.len();
    let pivot_index = len / 2;
    let pivot = (arr[pivot_index]).clone();
    let mut store_index = 0;

    arr.swap(pivot_index, len - 1);

    for i in 0..len - 1 {
        if arr[i] < pivot {
            arr.swap(i, store_index);
            store_index += 1;
        }
    }

    arr.swap(store_index, len - 1);
    store_index
}

#[cfg(test)]
mod tests {
    use crate::quick_sort;

    #[test]
    fn quick_sort_test() {
        let mut vec_numbers = vec![2, 1, 4, 5, 3];
        quick_sort(&mut vec_numbers);
        assert_eq!(vec_numbers, vec![1, 2, 3, 4, 5]);
    }
}
