use std::fmt::Display;
use std::io;
use std::marker::PhantomData;
use std::sync::Arc;
use std::time::Duration;
use tokio::signal;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut buffer = String::new();

    println!("Введите количество worker-ов: ");

    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;

    let workers_amount = buffer.trim().parse::<usize>().unwrap();
    let pool = WorkerPool::new(workers_amount);
    let pool_arc = Arc::new(Mutex::new(pool));
    let main_thread: JoinHandle<()>;
    {
        let pool_arc_clone = pool_arc.clone();
        // Спавним в отдельной таске данные в канал каждые 500 мс
        main_thread = tokio::spawn(async move {
            let mut counter = 0;

            loop {
                pool_arc_clone.lock().await.send_message(counter).await;
                pool_arc_clone.lock().await.send_message(counter).await;

                println!("Main thread: sent {}", counter);
                counter += 1;

                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        });
    }

    // Ожидаем нажатия Ctrl + C
    match signal::ctrl_c().await {
        Ok(()) => {
            // Завершение воркеров
            pool_arc.lock().await.shutdown().await;
            main_thread.abort();
        }
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
        }
    }

    // Ждем завершения основного поток (бесконечно)
    match main_thread.await {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Interrupted by Ctrl + C: {}", err);
        }
    };

    Ok(())
}

// Пул воркеров
pub struct WorkerPool<T>
where
    T: Send + Display + 'static,
{
    workers: Vec<Worker<T>>,
    sender: Option<flume::Sender<T>>,
}
impl<T> Drop for WorkerPool<T>
where
    T: Send + Display + 'static,
{
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.as_ref() {
                thread.abort();
            }
        }
    }
}

// Пул задач tokio
impl<T> WorkerPool<T>
where
    T: Send + Display + 'static,
{
    pub fn new(size: usize) -> WorkerPool<T> {
        let (sender, receiver) = flume::unbounded::<T>();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::<Worker<T>>::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        WorkerPool {
            workers,
            sender: Some(sender),
        }
    }

    // Завершение работы воркеров
    pub async fn shutdown(&mut self) {
        drop(self.sender.take());

        // Ожидание завершения каждого воркера
        for worker in &self.workers {
            if let Some(thread) = &worker.thread {
                let _ = thread.abort();
            }
        }

        println!("All workers have been shut down.");
    }

    // Отправка данных в канал MPMC
    pub async fn send_message(&self, message: T) {
        let _ = self.sender.as_ref().unwrap().send_async(message).await;
    }
}

// Воркер (обрабатывает поступающие сообщения)
struct Worker<T>
where
    T: Send + Display + 'static,
{
    id: usize,
    thread: Option<JoinHandle<()>>,
    phantom: PhantomData<T>,
}

impl<T> Worker<T>
where
    T: Send + Display + 'static,
{
    fn new(id: usize, receiver: Arc<Mutex<flume::Receiver<T>>>) -> Worker<T> {
        let thread = tokio::task::spawn(async move {
            loop {
                let message = receiver.lock().await.recv_async().await;
                match message {
                    Ok(message) => {
                        println!("Worker {id} got message: {message}");
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });

        Worker::<T> {
            id,
            thread: Some(thread),
            phantom: PhantomData,
        }
    }
}
