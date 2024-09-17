use std::{sync::Arc, thread, time::Duration};

fn main() {
    let counter = Arc::new(counter::Counter::new(0));

    let counter_clone = counter.clone();
    let handle1 = thread::spawn(move || {
        for _ in 0..10 {
            counter_clone.increment_by(1);
            thread::sleep(Duration::from_millis(500));
        }
    });

    let counter_clone_1 = counter.clone();
    let handle2 = thread::spawn(move || {
        for _ in 0..10 {
            counter_clone_1.increment_by(1);
            thread::sleep(Duration::from_millis(500));
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{}", counter.get());
}

mod counter {
    use std::sync::Mutex;

    // Структура счётчик для работы в конкурентной среде
    pub struct Counter<T: std::ops::AddAssign + Clone + Copy + std::fmt::Debug> {
        _inner_counter: Mutex<T>,
    }

    impl<T: std::ops::AddAssign + Clone + Copy + std::fmt::Debug> Counter<T> {
        pub fn new(value: T) -> Self {
            Counter {
                _inner_counter: Mutex::new(value),
            }
        }

        pub fn increment_by(&self, value: T) {
            let mut counter = self._inner_counter.lock().unwrap();
            *counter += value;
        }

        pub fn get(&self) -> T {
            return *self._inner_counter.lock().unwrap();
        }
    }
}
