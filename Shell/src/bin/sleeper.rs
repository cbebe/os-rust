use std::{env, process::exit, thread, time::Duration};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("USAGE: {} SECONDS", args[0]);
        exit(1);
    }
    thread::sleep(Duration::from_secs(args[1].parse().unwrap()))
}
