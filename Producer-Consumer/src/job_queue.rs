use std::sync::Mutex;
pub struct JobQueue {
    jobs: Mutex<Vec<u32>>,
    asked: Vec<u32>,
    received: Vec<u32>,
    completed: Vec<u32>,
}

impl JobQueue {
    pub fn new(n_threads: usize) -> Self {
        JobQueue {
            jobs: Mutex::new(Vec::with_capacity(n_threads * 2)),
            asked: Vec::with_capacity(n_threads),
            received: Vec::with_capacity(n_threads),
            completed: Vec::with_capacity(n_threads),
        }
    }
}
