use std::any::type_name;

fn main() {
    let number = 993;
    let string = "String";
    let vector = vec![1, 2, 3];
    let struct_test = Test {};

    print_type_of(&number);
    print_type_of(&string);
    print_type_of(&vector);
    print_type_of(&struct_test);
}

struct Test {}

// Определение типа переменной и вывод в консоль
fn print_type_of<T>(_: &T) {
    println!("Тип переменной: {}", type_name::<T>());
}
