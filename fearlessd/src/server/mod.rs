use std::net::{TcpListener, TcpStream};
use std::collections::LinkedList;
use std::string::String;
use std::io::{Read, Write};

pub mod tasks;

pub struct Server {
	listener: TcpListener,
	queue: LinkedList<tasks::Job>,
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

	pub fn listen_for_tasks(&mut self) {
		for stream in self.listener.incoming() {
			match stream {
				Ok(stream) => {
					println!("Got connection");
					let response: String = handle(stream);
					self.queue.push_back(tasks::Job { task: tasks::Task { name: String::from(response) }});

					println!("{:?}", self.queue.len());
				} 
				Err(stream) => { println!("Failure"); }
			}
		}
	}

	fn handle_job(&mut self, stream: TcpStream) -> tasks::Job {
		let response: String = handle(stream);

		println!("{}", response);

		tasks::Job { task: tasks::Task { name: String::from("Message") }}
	}

	pub fn add_job(&mut self, task: tasks::Job) {
		self.queue.push_back(task);
	}


}


fn handle(stream: TcpStream) -> String {
	let mut stream = stream;

	let mut response: String = String::from("");
	let _ = stream.write_all(b"HEY");

	let _ = stream.read_to_string(&mut response);

	response
}