use std::net::{TcpListener, TcpStream};
use std::thread;

mod utils;
use utils::request::Request;
use utils::response::{RawResponse, Response};

mod routes;
use routes::{get::get, set::set};

pub fn response404() -> Response {
	Response {
		status: 404,
		status_info: Some("Not Found".to_string()),
		body: "404".to_string(),
		content_type: "text/plain".to_string()
	}
}

pub fn default_port() -> i32 {
	7878
}

pub fn handle(stream: TcpStream) {
	let raw_req = RawResponse::read(&stream);
	let req = Request::new(raw_req);
	let res: Response = match &req.sub_url as &str {
		"/get" => get(req),
		"/set" => set(req),
		_ => response404()
	};
	RawResponse::write(stream, RawResponse::from_res(res));
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
