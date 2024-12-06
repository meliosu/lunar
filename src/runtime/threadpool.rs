use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
};

use crossbeam::channel::Sender;

type Job = Box<dyn FnOnce() + Send + Sync + 'static>;

enum Message {
    Quit,
    Job(Job),
}

pub struct ThreadPool {
    workers: Vec<JoinHandle<()>>,
    sender: Arc<Sender<Message>>,
    jobs: Arc<AtomicU64>,
}

impl ThreadPool {
    pub fn new() -> Self {
        let parallelism = thread::available_parallelism().unwrap().into();

        let (tx, rx) = crossbeam::channel::unbounded();

        let tx = Arc::new(tx);
        let rx = Arc::new(rx);

        let jobs = Arc::new(AtomicU64::new(0));

        let mut workers = Vec::new();

        for _ in 0..parallelism {
            workers.push(thread::spawn({
                let rx = rx.clone();
                let tx = tx.clone();
                let jobs = jobs.clone();

                move || {
                    while let Ok(message) = rx.recv() {
                        match message {
                            Message::Quit => break,
                            Message::Job(job) => {
                                job();
                                jobs.fetch_sub(1, Ordering::Relaxed);
                            }
                        }

                        if jobs.load(Ordering::Relaxed) == 0 {
                            for _ in 0..parallelism - 1 {
                                tx.send(Message::Quit).unwrap();
                            }

                            break;
                        }
                    }
                }
            }));
        }

        Self {
            workers,
            sender: tx.clone(),
            jobs: jobs.clone(),
        }
    }

    pub fn spawn<J>(&self, job: J)
    where
        J: FnOnce() + Send + Sync + 'static,
    {
        self.jobs.fetch_add(1, Ordering::Relaxed);
        self.sender.send(Message::Job(Box::new(job))).unwrap();
    }

    pub fn join(self) {
        for worker in self.workers {
            _ = worker.join();
        }
    }
}
