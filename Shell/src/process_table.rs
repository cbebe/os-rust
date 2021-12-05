use crate::{error::MyError, process::Process};
use std::{collections::HashMap, error::Error};

pub struct ProcessTable {
    processes: HashMap<i32, Process>,
}

impl ProcessTable {
    pub fn insert_job(&mut self, pid: i32, cmd: &String) {
        self.processes.insert(pid, Process::new(pid, &cmd));
    }

    pub fn process_ps_line(&mut self, buf: &String) -> Result<(), Box<dyn Error>> {
        let ps_tokens: Vec<&str> = buf.split(" ").collect();
        let pid = ps_tokens[0].parse::<i32>()?;
        let process = match self.processes.get_mut(&pid) {
            Some(p) => p,
            None => return Err(MyError::new("Process not found")),
        };
        let time_tokens: Vec<&str> = ps_tokens[2].split(":").collect();
        process.time = time_tokens[2].parse::<u32>()?;
        Ok(())
    }
}
