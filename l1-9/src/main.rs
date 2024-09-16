fn main() {
    let mut number: i64 = 0b1010_1010;

    println!("Исходное число \n{:#b}", number);

    set_bit_value(&mut number, 5, false);
    println!("{:#b}", number);

    set_bit_value(&mut number, 5, true);
    println!("{:#b}", number);
}

fn set_bit_value(value: &mut i64, bit_index: usize, bit_value: bool) {
    if bit_value {
        *value |= 1 << bit_index;
    } else {
        *value &= !(1 << bit_index);
    }
}
