use std::sync::Mutex;

pub struct JobQueue {
    pub jobs: Mutex<Vec<u32>>,
    pub asked: Vec<u32>,
    pub received: Vec<u32>,
    pub completed: Vec<u32>,
}

pub enum Counter {
    Asked,
    Received,
    Completed,
}

impl JobQueue {
    pub fn new(n_threads: usize) -> Self {
        JobQueue {
            jobs: Mutex::new(Vec::with_capacity(n_threads * 2)),
            asked: vec![0; n_threads],
            received: vec![0; n_threads],
            completed: vec![0; n_threads],
        }
    }

    pub fn increment(&mut self, i: usize, counter: Counter) {
        let vector = match counter {
            Counter::Asked => &mut self.asked,
            Counter::Received => &mut self.received,
            Counter::Completed => &mut self.completed,
        };
        let count = vector
            .get_mut(i - 1)
            .expect("Vector should be large enough to be indexed");
        *count = *count + 1;
    }

    pub fn produce(&mut self, n: u32) -> usize {
        let mut jobs = self.jobs.lock().unwrap();
        jobs.push(n);
        jobs.len()
    }

    pub fn consume(&mut self) -> (u32, usize) {
        let mut jobs = self.jobs.lock().unwrap();
        (jobs.pop().expect("Should not be empty"), jobs.len())
    }
}
