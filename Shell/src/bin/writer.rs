use nix::libc::{getrusage, rusage, RUSAGE_SELF};
use os_rust_shell::input::get_int;
use std::{
    fs::write,
    mem::zeroed,
    time::{Duration, Instant},
};

fn print_usage() {
    unsafe {
        let mut usage: rusage = zeroed();
        getrusage(RUSAGE_SELF, &mut usage);
        println!("User time = \t {} seconds", usage.ru_utime.tv_sec);
        println!("Sys  time = \t {} seconds", usage.ru_stime.tv_sec);
    }
}

fn main() {
    let max_duration = Duration::from_secs(get_int());
    let start = Instant::now();
    let vec: Vec<u16> = (0..16000).collect();
    let output = format!("{:?}", vec);

    while start.elapsed() < max_duration {
        write("/dev/null", &output).expect("Unable to write data");
    }
    print_usage();
}
