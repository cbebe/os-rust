use crate::{
    error::MyError, input::ParsedInput, process::print_resource_usage, process_table::ProcessTable,
};
use atty::Stream;
use lazy_static::lazy_static;
use nix::{sys::wait::wait, unistd::Pid};
use std::{
    error::Error,
    io::{self, Write},
    sync::Mutex,
};

lazy_static! {
    static ref COMMAND_TABLE: Mutex<ProcessTable> = Mutex::new(ProcessTable::new());
}

fn register_handler() {
    // register SIGCHLD handler that would reap dead children
}

fn job_operation<T>(input: &ParsedInput, func: &mut T) -> Result<(), Box<dyn Error>>
where
    T: FnMut(Pid) -> Result<(), Box<dyn Error>>,
{
    if let Some(pid) = input.get_int() {
        func(Pid::from_raw(pid))
    } else {
        Err(MyError::new("PID required"))
    }
}

fn parse_input(line: String, table: &mut ProcessTable) -> Result<(), Box<dyn Error>> {
    let input = ParsedInput::from(line.trim().to_owned());
    match &input.tokv[0][..] {
        "kill" => job_operation(&input, &mut |pid| table.kill_job(pid)),
        "resume" => job_operation(&input, &mut |pid| table.resume_job(pid)),
        "suspend" => job_operation(&input, &mut |pid| table.suspend_job(pid)),
        "wait" => job_operation(&input, &mut |pid| table.wait_job(pid)),
        "exit" => Ok(wait_and_exit()),
        "jobs" => table.show_jobs(),
        "sleep" => Ok(()),
        _ => Ok(()),
    }
}

fn wait_and_exit() {
    // wait for all children and then exit
    while let Ok(_) = wait() {}
    // print usage
    println!("Resources used");
    print_resource_usage();
    std::process::exit(0);
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let bytes = std::io::stdin().read_line(&mut input)?;
    // EOF, done with program
    if bytes == 0 {
        if atty::is(Stream::Stdin) {
            println!();
        }
        wait_and_exit();
    }
    parse_input(input, &mut COMMAND_TABLE.lock().unwrap())?;
    Ok(())
}

pub fn shell379() {
    register_handler();

    loop {
        if atty::is(Stream::Stdin) {
            print!("shell379> ");
            io::stdout().flush().ok().expect("Could not flush stdout");
        }
        if let Err(e) = run() {
            eprintln!("{}", e);
        }
    }
}
