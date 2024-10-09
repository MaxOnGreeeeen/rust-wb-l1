fn main() {
    // Статичный строковый литерал
    let s1 = "hello";
    // Динамический объект String
    let s2 = String::from("hello");
    // Срез строки s2
    let s3 = s2.as_str();

    // 5 байт (фактический размер строки)
    let size_of_s1 = std::mem::size_of_val(s1);
    // 8 байт - указатель, 8 байт - длина строки, 8 байт - capacity
    let size_of_s2 = std::mem::size_of_val(&s2);
    // 8 байт - указатель, 8 байт - длина строки
    let size_of_s3 = std::mem::size_of_val(&s3);

    println!("{:?}", size_of_s1); // 5
    println!("{:?}", size_of_s2); // 24
    println!("{:?}", size_of_s3); // 16
}
