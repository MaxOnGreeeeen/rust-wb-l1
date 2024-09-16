use std::io;
use std::io::Write;
use std::sync::Arc;
use std::thread;

fn main() -> io::Result<()> {
    let mut buffer = String::new();

    println!("Введите число элементов массива: ");

    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;

    let vec_size = buffer.trim().parse::<u32>().unwrap();
    let stdout: Arc<io::Stdout> = Arc::new(io::stdout());

    // Количество потоков для вычисления
    let num_threads = 4;
    //Количество элементов поддиапазона
    let chunk_elements_amount = (vec_size / num_threads as u32) as usize;

    let mut handles = vec![];

    // Параллельная обработка массива
    for i in 0..num_threads {
        let stdout_clone = stdout.clone();
        let start = i * chunk_elements_amount;
        let end = if i == num_threads - 1 {
            vec_size + 1
        } else {
            ((i + 1) as u32) * chunk_elements_amount as u32
        };

        let handle = thread::spawn(move || {
            (start..end as usize).for_each(|item| {
                writeln!(stdout_clone.lock(), "{}", item.pow(2)).unwrap();
            });
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}
