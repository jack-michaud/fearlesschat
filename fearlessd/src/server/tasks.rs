use std::string::String;
use std::collections::LinkedList;
use std::fmt;

pub struct Job {
	pub task: Task
}

pub struct Task {
	pub name: String
}

impl fmt::Display for Job {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.task.name)
    }
}