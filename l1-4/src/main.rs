use std::fmt::Display;
use std::marker::PhantomData;
use std::sync::mpsc::{self, RecvError};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;
use std::{io, thread};

fn main() -> io::Result<()> {
    let mut buffer = String::new();

    println!("Введите количество worker-ов: ");

    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;

    let workers_amount = buffer.trim().parse::<usize>().unwrap();
    let pool = WorkerPool::new(workers_amount);
    let pool_arc = Arc::new(pool);
    let main_thread: JoinHandle<()>;
    {
        let pool_arc_clone = pool_arc.clone();
        main_thread = thread::spawn(move || {
            let mut counter = 0;

            loop {
                pool_arc_clone.send_message(Some(counter));

                println!("Main thread: sent {}", counter);
                counter += 1;

                thread::sleep(Duration::from_millis(500));
            }
        });
    }

    // Ждем завершения основного поток (бесконечно)
    main_thread.join().unwrap();

    // Завершение воркеров
    for _ in 0..workers_amount {
        pool_arc.clone().send_message(None);
    }

    Ok(())
}

// Пул воркеров
pub struct WorkerPool<T>
where
    T: Send + Display + 'static,
{
    workers: Vec<Worker<T>>,
    sender: mpsc::Sender<Option<T>>,
}

impl<T> WorkerPool<T>
where
    T: Send + Display + 'static,
{
    pub fn new(size: usize) -> WorkerPool<T> {
        let (sender, receiver) = mpsc::channel::<Option<T>>();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::<Worker<T>>::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        WorkerPool { workers, sender }
    }

    pub fn send_message(&self, message: Option<T>) {
        self.sender.send(message).unwrap();
    }
}

// Воркер (обрабатывает поступающие задачи)
struct Worker<T>
where
    T: Send + Display + 'static,
{
    id: usize,
    thread: thread::JoinHandle<Result<(), Option<RecvError>>>,
    phantom: PhantomData<T>,
}

impl<T> Worker<T>
where
    T: Send + Display + 'static,
{
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Option<T>>>>) -> Worker<T> {
        let thread = thread::spawn(move || loop {
            let message = match receiver.lock().unwrap().recv() {
                Ok(Some(message)) => message,
                Ok(None) => {
                    println!("Worker {} is terminating.", id);
                    return Err(None);
                }
                Err(err) => {
                    println!("Error: {err}");
                    return Err(Some(err));
                }
            };

            println!("Worker {id} got message: {message}");
        });

        Worker::<T> {
            id,
            thread,
            phantom: PhantomData,
        }
    }
}
