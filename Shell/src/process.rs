use crate::error::MyError;
use nix::NixPath;
use nix::{
    fcntl::{open, OFlag},
    unistd::dup2,
};
use std::error::Error;
use std::fs::File;
#[cfg(unix)]
use std::os::unix::io::{IntoRawFd, RawFd};

enum ProcessStatus {
    Running,
    Suspended,
}

pub struct Process {
    pid: i32,
    time: u32,
    process_status: ProcessStatus,
    cmd: String,
}

fn redirect(
    file: String,
    permissions: OFlag,
    stat: i32,
    fileno: i32,
) -> Result<(), Box<dyn Error>> {
    // let file = open(, permissions, stat)?;
    // #[cfg(unix)]
    // let raw_fd: RawFd = file.into_raw_fd();
    // if raw_fd < 0 {
    //     return Err(MyError::new("Error opening file"));
    // }
    Ok(())
}
