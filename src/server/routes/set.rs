use crate::server::utils::{request::Request, response::Response};

pub fn set(_: Request) -> Response {
	Response {
		status: 200,
		status_info: Some("OK".to_string()),
		body: "{ \"test\": true }".to_string(),
		content_type: "application/json".to_string()
	}
}