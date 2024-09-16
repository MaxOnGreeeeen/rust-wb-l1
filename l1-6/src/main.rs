use std::io;
use std::sync::mpsc;
use tokio::time::{self, Duration};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut buffer = String::new();

    println!("Введите время работы программы: ");

    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;

    let duration_ms = buffer.trim().parse::<u64>().unwrap() * 1000;

    // Создаем таймер на заданное время
    let duration = Duration::from_millis(duration_ms);
    let mut interval = tokio::time::interval(Duration::from_millis(100));
    let mut counter = 0;

    let (sender, receiver) = mpsc::channel::<i32>();

    // Запускаем асинхронную задачу для чтения из канала
    tokio::spawn(async move {
        loop {
            let message = receiver.recv();
            match message {
                Ok(data) => {
                    println!("read data: {} {} time(s)", data, counter);
                    counter += 1;
                }
                Err(_) => {
                    break;
                }
            }
        }
        println!("channel closed");
    });

    let start = time::Instant::now();
    let mut data = 0;

    loop {
        tokio::select! {
            _ = interval.tick() => {
                // Отправляем сообщение в канал
                if let Err(_) = sender.send(data as i32) {
                    break;
                }
                data += 1;
            }
            _ = time::sleep_until(start + duration) => {
                // Закрываем канал по истечении времени
                drop(sender);
                break;
            }
        }
    }

    Ok(())
}
