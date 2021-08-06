use std::io::Write;
use std::net::TcpStream;

#[derive(Debug, Clone)]
pub struct Response {
	pub body: String,
	pub content_type: String,
	pub status: i16,
	pub status_info: Option<String>
}

impl Response {
	pub fn raw(&self) -> String {
		let status_info: String = match &self.status_info {
			Some(x) => format!(" {}", x),
			None => "".to_string()
		};

		format!(
			"HTTP/1.1 {}{}\r\nContent-Type: {}; charset=UTF-8\r\n\r\n{}\r\n",
			self.status, status_info, self.content_type, self.body
		)
	}

	pub fn write(&self, mut stream: TcpStream) {
		let response = self.raw();
		match stream.write(response.as_bytes()) {
			Ok(_) => (),
			Err(e) => println!("Failed sending response: {}", e)
		}
	}
}
