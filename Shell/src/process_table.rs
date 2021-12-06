use crate::{
    error::MyError,
    input::CmdOptions,
    process::{child_exec, parent_exec, print_resource_usage, Process, ProcessStatus},
};
use nix::{
    sys::{
        signal::{kill, Signal},
        wait::{waitpid, WaitPidFlag, WaitStatus},
    },
    unistd::{fork, ForkResult, Pid},
};
use std::{collections::HashMap, error::Error, process::Command, str};

pub struct ProcessTable {
    processes: HashMap<Pid, Process>,
}

impl ProcessTable {
    pub fn new() -> Self {
        ProcessTable {
            processes: HashMap::new(),
        }
    }

    fn get_job(&mut self, pid: &Pid) -> Result<&mut Process, Box<dyn Error>> {
        if let Some(process) = self.processes.get_mut(pid) {
            Ok(process)
        } else {
            Err(MyError::new("Process not found"))
        }
    }

    pub fn insert_job(&mut self, pid: Pid, cmd: &String) {
        self.processes.insert(pid, Process::new(&cmd));
    }

    pub fn process_ps_line(&mut self, buf: &str) -> Result<(), Box<dyn Error>> {
        let ps_tokens: Vec<&str> = buf.trim().split(" ").filter(|s| s.len() > 0).collect();
        let pid = Pid::from_raw(ps_tokens[0].parse::<i32>()?);
        if let Ok(process) = self.get_job(&pid) {
            let time_tokens: Vec<&str> = ps_tokens[2].split(":").collect();
            process.time = time_tokens[2].parse::<u32>()?;
        }
        Ok(())
    }

    pub fn get_ps_output(&mut self) -> Result<(), Box<dyn Error>> {
        let ps = Command::new("ps").output()?;
        let ps_out = String::from_utf8_lossy(&ps.stdout[..]);
        let ps_lines = ps_out.trim().split("\n");
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

    pub fn resume_job(&mut self, pid: Pid) -> Result<(), Box<dyn Error>> {
        let mut process = self.get_job(&pid)?;
        match process.status {
            ProcessStatus::Running => Err(MyError::new("Process already running")),
            ProcessStatus::Suspended => {
                kill(pid, Signal::SIGCONT)?;
                process.status = ProcessStatus::Running;
                Ok(())
            }
        }
    }

    pub fn reap_children(&mut self) {
        let mut live_children: HashMap<Pid, Process> = HashMap::new();
        self.processes.iter().for_each(|(pid, process)| {
            let pid_copy = pid.clone();
            if let Ok(WaitStatus::StillAlive) = waitpid(pid_copy, Some(WaitPidFlag::WNOHANG)) {
                live_children.insert(pid_copy, process.clone());
            }
        });
        self.processes = live_children
    }

    pub fn wait_job(&mut self, pid: Pid) -> Result<(), Box<dyn Error>> {
        let process = self.get_job(&pid)?;
        // don't let program hang by waiting for a suspended process
        if let ProcessStatus::Suspended = process.status {
            self.resume_job(pid)?;
        }
        waitpid(pid, None)?;
        Ok(())
    }

    pub fn kill_job(&mut self, pid: Pid) -> Result<(), Box<dyn Error>> {
        // no need to make sure that the process is there
        // since sending a signal to a non-existent process does nothing
        // we'll still let the user know, though
        self.get_job(&pid)?;
        kill(pid, Signal::SIGKILL)?;
        Ok(())
    }

    pub fn suspend_job(&mut self, pid: Pid) -> Result<(), Box<dyn Error>> {
        let mut process = self.get_job(&pid)?;
        match process.status {
            ProcessStatus::Suspended => Err(MyError::new("Process already suspended")),
            ProcessStatus::Running => {
                kill(pid, Signal::SIGSTOP)?;
                process.status = ProcessStatus::Suspended;
                Ok(())
            }
        }
    }

    pub fn new_job(&mut self, options: CmdOptions) -> Result<(), Box<dyn Error>> {
        if options.argv.len() == 0 {
            return Err(MyError::new("No command given"));
        }

        match unsafe { fork() } {
            Ok(ForkResult::Parent { child, .. }) => Ok(parent_exec(self, &options, child)),
            Ok(ForkResult::Child) => Ok(child_exec(&options)),
            Err(e) => Err(Box::new(e)),
        }
    }
}
