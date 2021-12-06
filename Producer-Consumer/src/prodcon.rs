use std::{convert, env, fmt, num, process};

#[derive(Debug, Clone)]
struct CmdLineError;

impl fmt::Display for CmdLineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let argv = env::args().collect::<Vec<String>>();
        write!(f, "USAGE: {} N_THREADS [LOG ID]", argv[0])
    }
}

impl convert::From<num::ParseIntError> for CmdLineError {
    fn from(_: num::ParseIntError) -> Self {
        CmdLineError
    }
}

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

pub fn main() {
    let (n_threads, filename) = match parse_args() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1)
        }
    };
}
