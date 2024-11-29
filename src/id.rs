use std::sync::atomic::{AtomicU64, Ordering};

pub struct Generator {
    counter: AtomicU64,
}

impl Generator {
    pub fn new() -> Self {
        Self {
            counter: AtomicU64::new(0),
        }
    }

    pub fn get(&self) -> u64 {
        self.counter.fetch_add(1, Ordering::Relaxed)
    }
}
