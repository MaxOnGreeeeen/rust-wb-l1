fn main() {
    let mut numbers = vec![1, 2, 3];
    numbers.remove(2);
    assert_eq!(numbers, [1, 2])
}
