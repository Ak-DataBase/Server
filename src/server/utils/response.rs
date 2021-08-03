use std::io::{Read, Write};
use std::net::TcpStream;

#[derive(Debug, Clone)]
pub struct Response {
	pub body: String,
	pub content_type: String,
	pub status: i16,
	pub status_info: Option<String>
}

pub struct RawResponse;

impl RawResponse {
	pub fn from_res(res: Response) -> String {
		let status_info: String = match res.status_info {
			Some(x) => format!(" {}", x),
			None => "".to_string()
		};

		format!(
			"HTTP/1.1 {}{}\r\nContent-Type: {}; charset=UTF-8\r\n\r\n{}\r\n",
			res.status, status_info, res.content_type, res.body
		)
	}

	pub fn write(mut stream: TcpStream, response: String) {
		match stream.write(response.as_bytes()) {
			Ok(_) => (),
			Err(e) => println!("Failed sending response: {}", e)
		}
	}

	pub fn read(mut stream: &TcpStream) -> String {
		let mut buf = [0u8; 4096];
		match stream.read(&mut buf) {
			Ok(_) => String::from_utf8_lossy(&buf).to_string(),
			Err(e) => panic!("Error reading stream: {}", e)
		}
	}
}
