use std::io;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::SystemTime;

static N_THREASHOLD: usize = 10_000;

fn main() -> io::Result<()> {
    let mut buffer = String::new();

    println!("Введите число элементов массива: ");

    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;

    let vec_size = buffer.trim().parse::<u32>().unwrap();
    let now = SystemTime::now();

    if (vec_size as usize) < N_THREASHOLD {
        let sum_of_squares: u64 = (1..vec_size + 1).map(|item| (item as u64).pow(2)).sum();
        println!(
            "Cумма квадратов от 0 до {} равна {}",
            vec_size, sum_of_squares
        );

        println!("Прошло времени:{:?} мс", now.elapsed().unwrap().as_millis());

        return Ok(());
    }

    // Доступное количество потоков для выполнения
    let num_threads = calculate_num_threads(vec_size as usize, N_THREASHOLD);
    //Количество элементов поддиапазона
    let chunk_elements_amount = (vec_size / num_threads as u32) as usize;

    let mut handles = vec![];

    // Канал для передачи вычисленных значений
    let (sender, receiver): (Sender<u128>, Receiver<u128>) = channel();

    // Параллельное вычисление суммы квадратов
    for i in 0..num_threads {
        let thread_sender = sender.clone();
        let start = i * chunk_elements_amount;
        let end = if i == num_threads - 1 {
            vec_size + 1
        } else {
            ((i + 1) as u32) * chunk_elements_amount as u32
        };

        // Запуск потока для вычисления суммы квадратов поддиапазона
        let handle = thread::spawn(move || {
            let result_sum_of_squares: u128 = (start..end as usize)
                .map(|item| (item as u64).pow(2) as u128)
                .sum();

            thread_sender
                .send(result_sum_of_squares)
                .expect("Sender error");
        });

        handles.push(handle);
    }

    let mut result = 0u128;
    for _ in 0..num_threads {
        let received_item = receiver.recv().expect("Не удалось получить результат");
        result += received_item;
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Результат сложения чисел от 1 до {} = {}", vec_size, result);
    println!("Прошло времени:{:?} мс", now.elapsed().unwrap().as_millis());

    Ok(())
}

// Возвращает оптимальное количество потоков для вычислений
fn calculate_num_threads(elements: usize, threshold: usize) -> usize {
    let available_threads = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    let num_threads = (elements / threshold).max(1).min(available_threads);

    num_threads
}
