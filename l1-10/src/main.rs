use std::{
    io,
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

fn main() -> io::Result<()> {
    let mut buffer = String::new();

    println!("Введите длину массива чисел: ");

    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;

    let vec_length = buffer.trim().parse::<usize>().unwrap();
    let vec_numbers: Vec<usize> = (1..vec_length + 1).map(|item| item).collect();

    let (sender, receiver): (Sender<usize>, Receiver<usize>) = mpsc::channel();
    let (sender_stdout, receiver_stdout): (Sender<usize>, Receiver<usize>) = mpsc::channel();

    // Запись чисел из массива в канал 1
    thread::spawn(move || {
        vec_numbers.iter().for_each(|number| {
            sender.send(*number).unwrap();
            thread::sleep(Duration::from_millis(500));
        });
    });

    // Чтение чисел из канала 1, отправка чисел возведённых в квадрат в канал 2
    let thread_square = thread::spawn(move || loop {
        let message = receiver.recv();
        match message {
            Ok(data) => {
                sender_stdout.send(data.pow(2)).unwrap();
            }
            Err(_) => {
                break;
            }
        }
    });

    // Чтение чисел из канала 2, вывод в stdout
    let thread_stdout = thread::spawn(move || loop {
        let message = receiver_stdout.recv();
        match message {
            Ok(data) => {
                println!("Thread 2 received data {}", data);
            }
            Err(_) => {
                break;
            }
        }
    });

    thread_square.join().unwrap();
    thread_stdout.join().unwrap();

    Ok(())
}
