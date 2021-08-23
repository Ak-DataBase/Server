use crate::utils::response::Response;

pub static JSON: &str = "application/json";

pub fn bad_request_str(body: String, content_type: &'static str) -> Response {
	Response {
		body,
		content_type: content_type.to_string(),
		status: 400,
		status_info: Some("BAD_REQUEST".to_string())
	}
}

pub fn ok_str(body: String, content_type: &'static str) -> Response {
	Response {
		body,
		content_type: content_type.to_string(),
		status: 200,
		status_info: Some("OK".to_string())
	}
}

pub fn ok(body: String) -> Response {
	ok_str(body, JSON)
}

pub fn bad_request(error: String) -> Response {
	let body = format!("{} \"error\": \"{}\" {}", '{', error, '}');
	bad_request_str(body, JSON)
}
