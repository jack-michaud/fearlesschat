extern crate libfearless;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::string::String;

pub mod server;


fn main() {
	let mut server: server::Server = server::Server::start("127.0.0.1", 3131);

	server.listen_for_jobs();

	while server.has_job() {
		println!("{}", execute_function(server.get_next_job()));
	}
}
	

fn execute_function(f: &Fn(i32, i32), arg: server::task::Arg) {
	f(arg.val1, arg.val2);
}
