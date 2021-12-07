use crate::job_queue::JobQueue;
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
    ask: u32,
    receive: u32,
    work: u32,
    sleep: u32,
}

impl Logger {
    pub fn new(filename: String) -> Self {
        Logger {
            // panic if we can't create file
            file: Mutex::new(File::create(filename).unwrap()),
            start: Instant::now(),
            ask: 0,
            receive: 0,
            work: 0,
            sleep: 0,
        }
    }

    pub fn log(&mut self, id: usize, event: Event) {
        use Event::*;
        let message = match event {
            Ask => format!("{:5}Ask", ""),
            Sleep(arg) => format!("{:5}{:-10} {:6}", "", "Sleep", arg),
            Complete(arg) => format!("{:5}{:-10} {:6}", "", "Complete", arg),
            Work { arg, q } => format!("Q={:2} {:-10} {:6}", q, "Work", arg),
            Receive { arg, q } => format!("Q={:2} {:-10} {:6}", q, "Receive", arg),
        };
        self.log_to_file(id, message, event)
    }

    fn log_to_file(&mut self, id: usize, message: String, event: Event) {
        use Event::*;
        let duration = self.start.elapsed().as_secs_f64();
        if let Ok(mut file) = self.file.lock() {
            // increment here to take advantage of the lock
            match event {
                Ask => self.ask += 1,
                Sleep(_) => self.sleep += 1,
                Work { .. } => self.work += 1,
                Receive { .. } => self.receive += 1,
                _ => {}
            }
            let message = format!("{:8.3} ID={:2} {}\n", duration, id, message);
            self.write_to_file(&mut file, message);
        }
    }

    #[inline(always)]
    fn write_to_file(&self, file: &mut File, message: String) {
        if let Err(_) = file.write(message.as_bytes()) {
            panic!()
        }
    }

    #[inline]
    fn print_value(&self, file: &mut File, label: &str, num: u32) {
        self.write_to_file(file, format!("{:4}{:-9} {:5}\n", "", label, num))
    }

    pub fn print_total_jobs(&self, queue: &JobQueue) {
        let duration = self.start.elapsed().as_secs_f64();
        // no one should be using the file anymore
        let mut file = self.file.lock().unwrap();
        self.write_to_file(&mut file, "Summary:\n".into());
        let completed = queue.get_completed();
        self.print_value(&mut file, "Work", self.work);
        self.print_value(&mut file, "Ask", self.ask);
        self.print_value(&mut file, "Receive", self.receive);
        self.print_value(&mut file, "Complete", completed.iter().sum());
        self.print_value(&mut file, "Sleep", self.sleep);
        for (i, val) in completed.iter().enumerate() {
            self.print_value(&mut file, &format!("Thread {:2}", i + 1)[..], *val);
        }
        let throughput = self.work as f64 / duration;
        let message = format!("Transactions per second: {:5.2}", throughput);
        self.write_to_file(&mut file, message);
    }
}
