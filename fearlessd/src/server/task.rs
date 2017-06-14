use std::string::String;

pub struct Task {
	name: String,
	func: fn(Arg),
}

pub struct Arg {
	val1: i32,
	val2: i32
}