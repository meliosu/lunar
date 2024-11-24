use std::{collections::HashMap, sync::RwLock};

use crate::value::Value;

pub struct ExecutionContext {
    pub store: Storage,
}

impl ExecutionContext {
    pub fn new() -> Self {
        Self {
            store: Storage::new(),
        }
    }
}

pub struct Storage {
    inner: RwLock<HashMap<String, Value>>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(HashMap::new()),
        }
    }

    pub fn insert(&self, key: String, value: Value) -> Option<Value> {
        self.inner.write().unwrap().insert(key, value)
    }

    pub fn get(&self, key: &str) -> Option<Value> {
        self.inner.read().unwrap().get(key).cloned()
    }

    pub fn contains(&self, key: &str) -> bool {
        self.inner.read().unwrap().contains_key(key)
    }
}
