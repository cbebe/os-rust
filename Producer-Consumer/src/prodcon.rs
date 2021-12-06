use crate::{error::CmdLineError, job_queue::JobQueue, logger::Logger};
use std::{env, process};

static mut JOB_QUEUE: Option<JobQueue> = None;
static mut LOGGER: Option<Logger> = None;

fn parse_args() -> Result<(usize, String), CmdLineError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(CmdLineError {});
    }
    let n_threads = args[1].parse()?;
    let filename = match args.len() {
        2 => "prodcon.log".to_owned(),
        _ => format!("prodcon.{}.log", args[2].parse::<usize>()?),
    };

    Ok((n_threads, filename))
}

/**
 * Unit of execution for consumer thread
 */
fn consoomer_func(i: usize) {}

/**
 * Unit of execution for producer thread
 */
fn producer_func() {}

fn print_summary() {}

fn init_globals(n_threads: usize, filename: String) {
    unsafe {
        JOB_QUEUE = Some(JobQueue::new(n_threads));
        LOGGER = Some(Logger::new(filename));
    }
}

pub fn main() {
    let (n_threads, filename) = match parse_args() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1)
        }
    };

    init_globals(n_threads, filename);

    let mut consoomers = vec![];
    for i in 0..n_threads {
        consoomers.push(std::thread::spawn(move || {
            consoomer_func(i);
        }))
    }
    producer_func();
    for consoomer in consoomers {
        let _ = consoomer.join();
    }
    print_summary();
}
