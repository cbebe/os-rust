use crate::process_table::ProcessTable;
use nix::{
    fcntl::{open, OFlag},
    libc::{STDIN_FILENO, STDOUT_FILENO},
    sys::{stat::Mode, wait::waitpid},
    unistd::{close, dup2, execvp, Pid},
};
use std::{error::Error, ffi::CString};

#[derive(Copy, Clone)]
pub enum ProcessStatus {
    Running,
    Suspended,
}

#[derive(Copy, Clone)]
enum ProcessType {
    Foreground,
    Background,
}

pub struct Process {
    pub pid: i32,
    pub time: u32,
    pub status: ProcessStatus,
    pub cmd: String,
}

impl Process {
    pub fn new(pid: i32, cmd: &String) -> Process {
        Process {
            pid,
            status: ProcessStatus::Running,
            cmd: cmd.clone(),
            time: 0,
        }
    }
}

pub struct CmdOptions {
    cmd: String,
    in_file: Option<String>,
    out_file: Option<String>,
    bg: ProcessType,
    argv: Vec<String>,
}

fn redirect(file: String, flags: OFlag, stat: Mode, fileno: i32) -> Result<(), Box<dyn Error>> {
    let file = open(file.as_str(), flags, stat)?;
    dup2(file, fileno)?;
    close(file)?;
    Ok(())
}

fn to_c_str(arr: Vec<String>) -> Vec<CString> {
    arr.into_iter()
        .map(|v| CString::new(v))
        .filter(|v| v.is_ok())
        // should not panic since all err are filtered
        .map(|v| v.unwrap())
        .collect()
}

pub fn child_exec(options: CmdOptions) {
    use std::process::exit;
    let write_bitmask: OFlag = OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_TRUNC;
    if let Some(file) = options.out_file {
        if let Err(_) = redirect(file, write_bitmask, Mode::S_IWUSR, STDOUT_FILENO) {
            exit(1)
        }
    }
    if let Some(file) = options.in_file {
        if let Err(_) = redirect(file, OFlag::O_RDONLY, Mode::S_IRUSR, STDIN_FILENO) {
            exit(1)
        }
    }
    let program = match CString::new(options.argv[0].clone().into_bytes()) {
        Ok(string) => string,
        Err(_) => exit(1),
    };
    if let Err(_) = execvp(&program, &to_c_str(options.argv)[..]) {
        exit(1)
    }
}

pub fn parent_exec(table: &mut ProcessTable, options: &CmdOptions, pid: i32) {
    match options.bg {
        ProcessType::Foreground => match waitpid(Pid::from_raw(pid), None) {
            Ok(_) => {}
            Err(_) => {}
        },
        ProcessType::Background => table.insert_job(pid, &options.cmd),
    }
}
