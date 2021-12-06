use crate::{
    error::CmdLineError,
    job_queue::{Counter, JobQueue},
    logger::{Event, Logger},
    tands::{sleep, trans},
};
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

fn use_static<T, R, D>(object: &mut Option<D>, func: T) -> R
where
    T: Fn(&mut D) -> R,
{
    match object {
        Some(ref mut obj) => func(obj),
        None => panic!(),
    }
}

fn use_queue<T, R>(func: T) -> R
where
    T: Fn(&mut JobQueue) -> R,
{
    unsafe { use_static(&mut JOB_QUEUE, func) }
}

fn use_logger<T, R>(func: T) -> R
where
    T: Fn(&mut Logger) -> R,
{
    unsafe { use_static(&mut LOGGER, func) }
}

/**
 * Unit of execution for consumer thread
 */
fn consoomer_func(i: usize) {
    loop {
        use_queue(|q| q.increment(i, Counter::Asked));
        // log ask
        use_logger(|l| l.log(i, Event::Ask));
        // consume queue
        let (arg, q) = use_queue(|q| q.consume());

        // return if no jobs

        // receive job, log
        use_queue(|q| q.increment(i, Counter::Received));
        use_logger(|l| l.log(i, Event::Receive { arg, q }));

        // do job
        trans(arg);

        use_queue(|q| q.increment(i, Counter::Completed));
        // log job completed
        use_logger(|l| l.log(i, Event::Complete(arg)));
    }
}

/**
 * Unit of execution for producer thread
 */
fn producer_func() {
    loop {
        let mut input = String::new();
        if let Ok(bytes) = std::io::stdin().read_line(&mut input) {
            if bytes == 0 {
                break;
            }
            let mut chars = input.chars();
            match chars.next().expect("Should have a char") {
                'S' => {
                    // log sleep
                    let arg = chars.as_str().trim().parse().expect("Sleep parse error");
                    use_logger(|l| l.log(0, Event::Sleep(arg)));
                    // add number of sleep
                    sleep(arg);
                }
                'T' => {
                    let arg = chars.as_str().trim().parse().expect("Trans parse error");
                    // log work
                    // add number of work
                    let q = use_queue(|q| q.produce(arg));
                    use_logger(|l| l.log(0, Event::Work { arg, q }));
                }
                _ => panic!(),
            }
        }
    }
}

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
    for i in 1..=n_threads {
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
