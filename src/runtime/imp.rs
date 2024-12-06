use std::{
    borrow::Borrow,
    cell::RefCell,
    collections::HashMap,
    ffi::c_void,
    sync::{LazyLock, Mutex, RwLock},
};

use super::{
    api::{Block, Df},
    id::Generator,
    storage::Storage,
    threadpool::ThreadPool,
};

pub static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| Runtime::new());

thread_local! {
    static CURRENT: RefCell<Option<Coroutine>> = RefCell::new(None);
}

pub struct Entry {
    df: Option<Df>,
    waiting: Vec<Coroutine>,
}

#[derive(Clone)]
pub struct Coroutine {
    id: u64,
    block: Block,
    ctx: usize,
}

pub struct Runtime {
    pool: ThreadPool,
    storage: Mutex<HashMap<u64, Entry>>,
    df_id_gen: Generator,
    cf_id_gen: Generator,
}

impl Runtime {
    fn new() -> Self {
        Self {
            pool: ThreadPool::new(),
            storage: Mutex::new(HashMap::new()),
            df_id_gen: Generator::new(),
            cf_id_gen: Generator::new(),
        }
    }

    pub fn alloc_dfid(&self) -> u64 {
        self.df_id_gen.get()
    }

    pub fn request(&self, dfid: u64) -> Option<Df> {
        let mut storage = self.storage.lock().unwrap();

        let current = CURRENT.with_borrow(|v| v.clone().unwrap());

        match storage.get_mut(&dfid) {
            Some(entry) => {
                if let Some(df) = &entry.df {
                    Some(df.clone())
                } else {
                    entry.waiting.push(current);
                    None
                }
            }

            None => {
                storage.insert(
                    dfid,
                    Entry {
                        df: None,
                        waiting: vec![current],
                    },
                );

                None
            }
        }
    }

    pub fn wait(&self, dfid: u64) -> Df {
        loop {
            let storage = self.storage.lock().unwrap();

            if let Some(entry) = storage.get(&dfid) {
                if let Some(df) = &entry.df {
                    return df.clone();
                }
            }
        }
    }

    pub fn submit(&self, df: Df) {
        let mut storage = self.storage.lock().unwrap();
        let cfid = CURRENT.with_borrow(|v| v.clone().unwrap());

        if let Some(entry) = storage.get_mut(&df.id) {
            for waiting in std::mem::take(&mut entry.waiting) {
                self.pool.spawn(|| exec_coroutine(waiting));
            }
        } else {
            storage.insert(
                df.id,
                Entry {
                    df: Some(df.clone()),
                    waiting: Vec::new(),
                },
            );
        }
    }

    pub fn spawn(&self, block: Block, ctx: usize) {
        let coroutine = Coroutine {
            id: self.cf_id_gen.get(),
            block,
            ctx,
        };

        self.pool.spawn(|| exec_coroutine(coroutine));
    }
}

fn exec_coroutine(coroutine: Coroutine) {
    CURRENT.with_borrow_mut(|curr| *curr = Some(coroutine.clone()));

    let action = unsafe { (coroutine.block)(coroutine.ctx as *mut _) };

    match action {
        super::api::Action::Wait => {}
        super::api::Action::Exit => {}
    }
}
