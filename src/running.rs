
use lazy_static::lazy_static;

use std::process::Child;

// Handles the processes that are called ending in "&"
pub struct ProcessHandler {
    procs: Vec<RunningProcess>,
    next_job_num: usize
}

#[derive(Debug)]
struct RunningProcess {
    start_command: String,
    job_number: usize,
    jobs: CommandStructure,
}


impl std::fmt::Display for RunningProcess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "job number {} ({:?})", self.job_number, self.start_command)
    }
}

