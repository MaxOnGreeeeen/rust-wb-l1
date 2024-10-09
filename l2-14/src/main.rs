fn main() {
    // Создание mpsc канала
    let (tx, rv) = std::sync::mpsc::channel::<i32>();

    // Создание потока для передачи данных
    let handle = std::thread::spawn(move || {
        for i in 0..10 {
            // Отправка в канал сообщений от 0 до 9
            tx.send(i).unwrap();
        }
    });

    // Ожидание завершения потока
    handle.join().unwrap();

    // Итератор, блокирующий основной поток, для приёма сообщений
    for i in rv.iter() {
        // Вывод сообщений из канала в stdout
        println!("{i:?}");
    }
}
