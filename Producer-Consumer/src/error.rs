use std::{convert, env, fmt, num};
#[derive(Debug, Clone)]
pub struct CmdLineError;

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
