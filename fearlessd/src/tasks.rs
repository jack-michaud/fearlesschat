
use server::task;
use std::collections::vec::Vec


pub MESSAGE = task::Task { 
	name: "ADD", 
	func: |i1: i32, i2: i32| { i1 + i2 },
	args: Arg {val1: 1, val2: 2},
} 