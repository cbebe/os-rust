use atty::Stream;
use nix::sys::wait::wait;
use os_rust_shell::{process::print_resource_usage, process_table::ProcessTable};
use std::error::Error;
use std::io::{self, Write};

fn register_handler() {
    // register SIGCHLD handler that would reap dead children
}

fn parse_input(input: String) -> Result<(), Box<dyn Error>> {
    // parse tokens like &, >, and <
    // also get regular inputs and stuff

    // match comand

    // exit

    // jobs
    if input.trim() == "jobs" {
        let mut p_table = ProcessTable::new();
        p_table.show_jobs()?;
    }

    // kill <pid>

    // resume <pid>

    // suspend <pid>

    // wait <pid>

    // sleep <n>

    // run shell command

    Ok(())
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
    parse_input(input)?;
    Ok(())
}

fn main() {
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
