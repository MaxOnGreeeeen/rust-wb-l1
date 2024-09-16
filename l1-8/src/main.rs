use std::{
    sync::Arc,
    thread::{self, JoinHandle},
};

use dashmap::DashMap;

fn main() {
    concurrent_hash_map();
    concurrent_dash_map();
}

// Конкурентная запись в HashMap с использованием структуры ConcurrentHashMap
fn concurrent_hash_map() {
    let concurrent_hash_map: Arc<concurrent_hash_map::ConcurrentHashMap<i32, i32>> =
        Arc::new(concurrent_hash_map::ConcurrentHashMap::new());

    // Создаем thread-ы
    let mut handles: Vec<JoinHandle<()>> = vec![];
    (0..5).for_each(|i| {
        let concurrent_hash_map = Arc::clone(&concurrent_hash_map);
        let handle = thread::spawn(move || {
            // Конкурентная запись данных
            concurrent_hash_map.insert(i, i);
        });
        handles.push(handle);
    });

    // Ждем завершения всех thread-ов
    for handle in handles {
        handle.join().unwrap();
    }

    for (key, value) in concurrent_hash_map.iter() {
        println!("Key: {key}, value: {value}");
    }
}

// Конкурентная запись в HashMap с использованием DashMap
fn concurrent_dash_map() {
    let concurrent_hash_map: Arc<DashMap<i32, i32>> = Arc::new(DashMap::new());

    // Создаем thread-ы
    let mut handles: Vec<JoinHandle<()>> = vec![];
    (0..5).for_each(|i| {
        let concurrent_hash_map = Arc::clone(&concurrent_hash_map);
        let handle = thread::spawn(move || {
            // Конкурентная запись данных
            concurrent_hash_map.insert(i, i);
        });
        handles.push(handle);
    });

    // Ждем завершения всех thread-ов
    for handle in handles {
        handle.join().unwrap();
    }

    concurrent_hash_map.iter().for_each(|key_value_pair| {
        let key = key_value_pair.key();
        let value = key_value_pair.value();

        println!("Key: {key}, value: {value}");
    })
}

mod concurrent_hash_map {
    use std::{collections::HashMap, sync::Mutex};

    // Структура для конкурентной работы с HashMap (через std::sync::Mutex)
    pub struct ConcurrentHashMap<K, V>
    where
        K: Eq + std::hash::Hash,
    {
        hash_map: Mutex<HashMap<K, V>>,
    }
    impl<K, V> ConcurrentHashMap<K, V>
    where
        K: Eq + std::hash::Hash,
    {
        pub fn new() -> Self {
            let hash_map: Mutex<HashMap<K, V>> = Mutex::new(HashMap::new());
            Self { hash_map }
        }

        pub fn insert(&self, k: K, value: V) {
            self.hash_map.lock().unwrap().insert(k, value);
        }

        pub fn get(&self, k: &K) -> Option<V>
        where
            V: Clone,
        {
            self.hash_map.lock().unwrap().get(k).cloned()
        }

        // Метод для создания итератора по hash_map
        pub fn iter(&self) -> Vec<(K, V)>
        where
            K: Clone,
            V: Clone,
        {
            let guard = self.hash_map.lock().unwrap();
            guard.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
        }
    }
}
