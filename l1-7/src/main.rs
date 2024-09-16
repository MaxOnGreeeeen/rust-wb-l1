use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

use tokio::time::{sleep, Duration};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    tokio_tasks_stop_example().await;
    thread_stop_example();
}

// Использование закрытия каналов для остановки thread-ов
fn thread_stop_example() {
    let (sender, receiver) = mpsc::channel();
    let reciever_arc = Arc::new(Mutex::new(receiver));

    // Создаем thread-ы
    let handles: Vec<_> = (0..5)
        .map(|i| {
            let receiver = reciever_arc.clone();
            thread::spawn(move || loop {
                match receiver.lock().unwrap().recv() {
                    Ok(message) => println!("Thread {} received: {}", i, message),
                    Err(_) => {
                        println!("Thread {} shutting down", i);
                        break;
                    }
                }
            })
        })
        .collect();

    // Отправляем сообщения
    for i in 0..10 {
        sender.send(i).unwrap();
        thread::sleep(Duration::from_millis(100));
    }

    // Закрываем канал, чтобы воркеры завершились
    drop(sender);

    // Ждем завершения всех thread-ов
    for handle in handles {
        handle.join().unwrap();
    }
}

// Использование tokio_util::CancellationToken для остановки tokio task'ов
async fn tokio_tasks_stop_example() {
    let token = CancellationToken::new();

    let token1 = token.clone();
    let token2 = token.clone();

    let task1 = tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = token1.cancelled() => {
                        println!("Task 1 is cancelling...");
                        break;
                }
                _ = tokio::time::sleep(Duration::from_secs(10)) => {
                    println!("Task 1 completed normally");
                    break;
                }
            }
        }
        println!("Task 1 is cleaning up");
    });

    let task2 = tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = token2.cancelled() => {
                        println!("Task 2 is cancelling...");
                        break;
                }
                _ = tokio::time::sleep(Duration::from_secs(10)) => {
                    println!("Task 2 completed normally");
                    break;
                }
            }
        }
        println!("Task 2 is cleaning up");
    });

    sleep(Duration::from_millis(1000)).await;

    // Посылаем сигнал об отмене
    token.cancel();

    // Ожидание завершения task-ов
    let _ = tokio::join!(task1, task2);
}
