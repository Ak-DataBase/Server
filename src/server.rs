use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::{Duration, Instant};

use crate::db::{get::get, set::set};
use crate::utils::request::Request;
use crate::utils::response::Response;

pub fn response404() -> Response {
	Response {
		status: 404,
		status_info: Some("Not Found".to_string()),
		body: "404".to_string(),
		content_type: "text/plain".to_string()
	}
}

pub fn read(mut stream: &TcpStream) -> String {
	let mut buf = [0u8; 4096];
	match stream.read(&mut buf) {
		Ok(_) => String::from_utf8_lossy(&buf).to_string(),
		Err(e) => panic!("Error reading stream: {}", e)
	}
}

pub const DEFAULT_PORT: &'static str = "7878";

pub fn handle(stream: TcpStream) {
	let before = Instant::now();
	let req = Request::new(read(&stream));
	let res: Response = match &req.sub_url as &str {
		"/get" => get(req.clone()),
		"/set" => set(req.clone()),
		_ => response404()
	};
	res.write(stream);

	println!(
		"{} {} - Elapsed time: {:?}",
		req.method,
		req.sub_url,
		before.elapsed()
	);
}

pub fn run(port: i32) {
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
}
