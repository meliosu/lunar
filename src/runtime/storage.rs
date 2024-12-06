use std::{collections::HashMap, hash::Hash, sync::Mutex};

pub struct Storage<K, V> {
    inner: Mutex<HashMap<K, V>>,
}

impl<K, V> Storage<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(HashMap::new()),
        }
    }

    pub fn insert(&self, key: K, value: V) -> Option<V> {
        self.inner.lock().unwrap().insert(key, value)
    }

    pub fn remove(&self, key: &K) -> Option<V> {
        self.inner.lock().unwrap().remove(key)
    }

    pub fn get(&self, key: &K) -> Option<V> {
        self.inner.lock().unwrap().get(key).cloned()
    }
}
