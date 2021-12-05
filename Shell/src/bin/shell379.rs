use atty::Stream;
use nix::libc::{getrusage, rusage, RUSAGE_SELF};
use std::io::{self, Write};
use std::mem::zeroed;

fn register_handler() {
    // register SIGCHLD handler that would reap dead children
}

fn parse_input(input: String) {
    println!("{}", input);
    // parse tokens like &, >, and <
    // also get regular inputs and stuff

    // match comand

    // exit

    // jobs

    // kill <pid>

    // resume <pid>

    // suspend <pid>

    // wait <pid>

    // sleep <n>

    // run shell command
}

fn print_resource_usage() {
    unsafe {
        let mut usage: rusage = zeroed();
        getrusage(RUSAGE_SELF, &mut usage);
        println!("User time = \t {} seconds", usage.ru_utime.tv_sec);
        println!("Sys  time = \t {} seconds", usage.ru_stime.tv_sec);
    }
}

fn wait_and_exit() {
    // wait for all children and then exit
    // print usage
    println!("Resources used");
    print_resource_usage();
    std::process::exit(0);
}

fn run() {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(bytes) => {
            if bytes == 0 {
                if atty::is(Stream::Stdin) {
                    println!();
                }
                wait_and_exit();
            }
            parse_input(input);
        }
        Err(e) => eprintln!("{:?}", e),
    }
}

fn main() {
    register_handler();

    loop {
        if atty::is(Stream::Stdin) {
            print!("shell379> ");
            io::stdout().flush().ok().expect("Could not flush stdout");
        }
        run();
    }
}
