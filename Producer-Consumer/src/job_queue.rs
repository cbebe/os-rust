use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};

pub struct JobQueue {
    n_consumers: usize,
    jobs: Mutex<VecDeque<Option<u32>>>,
    completed: Vec<u32>,
    full: Condvar,
    empty: Condvar,
}

impl JobQueue {
    pub fn new(n_threads: usize) -> Self {
        JobQueue {
            jobs: Mutex::new(VecDeque::with_capacity(n_threads * 2)),
            completed: vec![0; n_threads],
            full: Condvar::new(),
            empty: Condvar::new(),
            n_consumers: n_threads,
        }
    }

    pub fn get_completed(&self) -> &Vec<u32> {
        &self.completed
    }

    pub fn increment(&mut self, i: usize) {
        let count = self
            .completed
            .get_mut(i - 1)
            .expect("Vector should be large enough to be indexed");
        *count = *count + 1;
    }

    pub fn produce(&mut self, n: Option<u32>) -> usize {
        let mut jobs = self.jobs.lock().unwrap();
        while jobs.len() == jobs.capacity() {
            jobs = self.full.wait(jobs).unwrap();
        }
        jobs.push_back(n);
        self.empty.notify_one();
        jobs.len()
    }

    pub fn consume(&mut self) -> (Option<u32>, usize) {
        let mut jobs = self.jobs.lock().unwrap();
        while jobs.len() == 0 {
            jobs = self.empty.wait(jobs).unwrap();
        }
        let n = jobs.pop_front().flatten();
        self.full.notify_one();
        (n, jobs.len())
    }

    pub fn end(&mut self) {
        for _ in 0..(self.n_consumers) {
            self.produce(None);
        }
    }
}
