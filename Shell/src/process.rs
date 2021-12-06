use crate::{
    input::{CmdOptions, ProcessType},
    process_table::ProcessTable,
};
use nix::{
    fcntl::{open, OFlag},
    libc::{_exit, getrusage, rusage, RUSAGE_CHILDREN, STDIN_FILENO, STDOUT_FILENO},
    sys::{stat::Mode, wait::waitpid},
    unistd::{close, dup2, execvp, Pid},
};
use std::{error::Error, ffi::CString, fmt, mem::zeroed, result};

#[derive(Copy, Clone)]
pub enum ProcessStatus {
    Running,
    Suspended,
}

impl fmt::Display for ProcessStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> result::Result<(), fmt::Error> {
        let char = match self {
            ProcessStatus::Running => 'R',
            ProcessStatus::Suspended => 'S',
        };
        write!(f, "{}", char)
    }
}

#[derive(Clone)]
pub struct Process {
    pub time: u32,
    pub status: ProcessStatus,
    pub cmd: String,
}

impl Process {
    pub fn new(cmd: &String) -> Self {
        Process {
            status: ProcessStatus::Running,
            cmd: cmd.clone(),
            time: 0,
        }
    }
}

pub fn print_resource_usage() {
    unsafe {
        let mut usage: rusage = zeroed();
        getrusage(RUSAGE_CHILDREN, &mut usage);
        println!("User time = \t {} seconds", usage.ru_utime.tv_sec);
        println!("Sys  time = \t {} seconds", usage.ru_stime.tv_sec);
    }
}

fn redirect(file: &String, flags: OFlag, stat: Mode, fileno: i32) -> Result<(), Box<dyn Error>> {
    let file = open(file.as_str(), flags, stat)?;
    dup2(file, fileno)?;
    close(file)?;
    Ok(())
}

fn to_c_str(arr: &Vec<String>) -> Vec<CString> {
    arr.into_iter()
        .map(|v| CString::new(v.clone()))
        .filter(|v| v.is_ok())
        // should not panic since all err are filtered
        .map(|v| v.unwrap())
        .collect()
}

fn try_child_exec(options: &CmdOptions) -> Result<(), Box<dyn Error>> {
    let write_bitmask: OFlag = OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_TRUNC;
    if let Some(file) = &options.out_file {
        let file_mode = Mode::from_bits_truncate(0o644);
        redirect(file, write_bitmask, file_mode, STDOUT_FILENO)?;
    }
    if let Some(file) = &options.in_file {
        redirect(file, OFlag::O_RDONLY, Mode::S_IRUSR, STDIN_FILENO)?;
    }
    let program = CString::new(options.argv[0].clone().into_bytes())?;
    execvp(&program, &to_c_str(&options.argv)[..])?;
    Ok(())
}

pub fn child_exec(options: &CmdOptions) {
    if let Err(e) = try_child_exec(options) {
        eprintln!("{}", e);
        unsafe { _exit(1) }
    }
}

pub fn parent_exec(table: &mut ProcessTable, options: &CmdOptions, pid: Pid) {
    match options.bg {
        ProcessType::Foreground => match waitpid(pid, None) {
            Ok(_) => {}
            Err(_) => {}
        },
        ProcessType::Background => table.insert_job(pid, &options.cmd),
    }
}
