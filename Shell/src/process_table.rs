use crate::{
    error::MyError,
    process::{print_resource_usage, Process},
};
use std::{collections::HashMap, error::Error, process::Command, str};

pub struct ProcessTable {
    processes: HashMap<i32, Process>,
}

impl ProcessTable {
    pub fn new() -> ProcessTable {
        ProcessTable {
            processes: HashMap::new(),
        }
    }
    pub fn insert_job(&mut self, pid: i32, cmd: &String) {
        self.processes.insert(pid, Process::new(&cmd));
    }

    pub fn process_ps_line(&mut self, buf: &str) -> Result<(), Box<dyn Error>> {
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

    pub fn get_ps_output(&mut self) -> Result<(), Box<dyn Error>> {
        let ps = Command::new("ps").output()?;
        let ps_out = String::from_utf8_lossy(&ps.stdout[..]);
        let ps_lines = ps_out.split("\n");
        for line in ps_lines.skip(1) {
            self.process_ps_line(line)?;
        }
        Ok(())
    }

    pub fn show_jobs(&mut self) -> Result<(), Box<dyn Error>> {
        println!("Running processes:");
        if self.processes.len() > 0 {
            self.get_ps_output()?;
            println!(" #      PID S SEC COMMAND");
            for (i, (pid, process)) in self.processes.iter().enumerate() {
                println!(
                    "{:2}: {:7} {}{:4} {}",
                    i, pid, process.status, process.time, process.cmd
                );
            }
        }
        println!("Processes =\t {} active", self.processes.len());
        println!("Completed processes:");
        print_resource_usage();
        Ok(())
    }
}
