use nix::libc::{getrusage, rusage, RUSAGE_SELF};
use os_rust_shell::input::get_int;
use std::{
    fs::write,
    io::Result,
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

fn main() -> Result<()> {
    let max_duration = Duration::from_secs(get_int());
    let start = Instant::now();

    while start.elapsed() < max_duration {
        write("/dev/null", b"1")?;
    }
    print_usage();
    Ok(())
}
