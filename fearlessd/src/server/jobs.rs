use std::string::String;
use std::collections::LinkedList;
use std::fmt;

use server::task::{Task, Arg};

pub struct Job {
	pub task: Task,
	pub arg: Arg
}


impl fmt::Display for Job {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.task.name)
    }
}