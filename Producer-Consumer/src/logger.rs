use std::{fs::File, sync::Mutex, time::Instant};

pub struct Logger {
    file: Mutex<File>,
    start: Instant,
}

impl Logger {
    pub fn new(filename: String) -> Self {
        Logger {
            // panic if we can't create file
            file: Mutex::new(File::create(filename).unwrap()),
            start: Instant::now(),
        }
    }
}
