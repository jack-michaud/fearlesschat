use std::net::{TcpListener, TcpStream};
use std::collections::LinkedList;
use std::string::String;
use std::io::{Read, Write};

pub mod jobs;
pub mod task;

pub struct Server {
	listener: TcpListener,
	queue: LinkedList<jobs::Job>,
}

impl Server {
	
	pub fn start(host: &str, port: u16) -> Server {
		Server { 
			listener: TcpListener::bind(format!("{}:{}", host, port)).unwrap(),
			queue: LinkedList::new()
		}
	}

	pub fn listen(&mut self, f: &Fn(TcpStream)) {
		for stream in self.listener.incoming() {
			match stream {
				Ok(stream) => {
					println!("Got connection");
					let _ = f(stream);
				} 
				Err(stream) => { println!("Failure"); }
			}
		}
	}

	pub fn listen_for_jobs(&mut self) {
		for stream in self.listener.incoming() {
			match stream {
				Ok(stream) => {
					println!("Got connection");
					let response: String = handle(stream);
					self.queue.push_back(jobs::Job { 
						task: MESSAGE,
						arg: Arg { val1: 1, val2: 2 },
					});

					println!("{:?}", self.queue.len());
				} 
				Err(stream) => { println!("Failure"); }
			}
		}
	}

	pub fn get_next_job(&mut self) -> Option<jobs::Job> {
		self.queue.pop_front()
	}

	pub fn has_job(&mut self) -> bool {
		self.queue.is_empty()
	}


}


fn handle(stream: TcpStream) -> String {
	let mut stream = stream;

	let mut response: String = String::from("");
	let _ = stream.write_all(b"HEY");

	let _ = stream.read_to_string(&mut response);

	response
}