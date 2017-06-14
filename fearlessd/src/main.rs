extern crate libfearless;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::string::String;

pub mod server;


fn main() {
	let mut server: server::Server = server::Server::start("127.0.0.1", 3131);

	server.listen_for_tasks();
}
	
