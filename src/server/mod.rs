use std::net::{TcpListener, TcpStream};
use std::thread;

mod utils;
use utils::request::Request;
use utils::response::{RawResponse, Response};

pub fn default_port() -> i32 {
	7878
}

pub fn handle(stream: TcpStream) {
	let raw_req = RawResponse::read(&stream);
	let req = Request::new(raw_req);
	let res = Response {
		body: "test".to_string(),
		content_type: "text/plain".to_string(),
		status: 200,
		status_info: Some("OK".to_string())
	};
	println!("{:?}\n{:?}", req, res);
	let response_str = RawResponse::from_res(res);
	RawResponse::write(stream, response_str);
}

pub fn run(port: i32) -> anyhow::Result<()> {
	println!("Starting server...");

	let addr = format!("127.0.0.1:{}", port);
	let addr_clone = addr.clone();
	let listener = TcpListener::bind(addr).unwrap();

	println!("Server listening on addr {}", addr_clone);
	for stream in listener.incoming() {
		match stream {
			Ok(stream) => {
				thread::spawn(|| handle(stream));
			}
			Err(e) => {
				println!("Unable to connect: {}", e);
			}
		}
	}

	Ok(())
}
