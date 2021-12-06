use std::{fs::File, io::Write, sync::Mutex, time::Instant};

pub enum Event {
    Ask,
    Sleep(u32),
    Complete(u32),
    Work { arg: u32, q: usize },
    Receive { arg: u32, q: usize },
}

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

    pub fn log(&self, id: usize, event: Event) {
        use Event::*;
        let message = match event {
            Ask => format!("{:5}Ask", ""),
            Sleep(arg) => format!("{:5}{:-10} {:6}", "", "Sleep", arg),
            Complete(arg) => format!("{:5}{:-10} {:6}", "", "Complete", arg),
            Work { arg, q } => format!("Q={:2} {:-10} {:6}", q, "Work", arg),
            Receive { arg, q } => format!("Q={:2} {:-10} {:6}", q, "Receive", arg),
        };
        self.log_to_file(id, message)
    }

    fn log_to_file(&self, id: usize, message: String) {
        let duration = self.start.elapsed().as_secs_f64();
        if let Ok(mut file) = self.file.lock() {
            match file.write(format!("{:8.3} ID={:2} {}\n", duration, id, message).as_bytes()) {
                Ok(_) => {}
                Err(_) => panic!(),
            }
        }
    }
}
