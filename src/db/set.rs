use super::db_struct::DB;
use super::utils::{bad_request, ok, JSON};
use crate::utils::{request::Request, response::Response};
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct SetBody {
	db_id: String,
	key: String,
	value: Value
}

pub fn set(req: Request) -> Response {
	if req.method != "POST" {
		return bad_request(format!("Expected method POST, found {}", req.method));
	}

	if req.post_info == None {
		return bad_request("Expected a body".to_string());
	}

	let post_info = match req.post_info {
		None => panic!(),
		Some(x) => x
	};

	let raw_body = post_info.body;
	let content_type = post_info.content_type;

	if content_type != JSON {
		return bad_request(format!(
			"Expected content type of {}, found {}.",
			JSON, content_type
		));
	}

	let body = match serde_json::from_str::<SetBody>(&raw_body) {
		Ok(x) => x,
		Err(e) => return bad_request(format!("Error parsing JSON: {}", e))
	};

	let id = body.db_id;

	if !DB::exists(id.clone()) {
		return bad_request("Specified DB id does not exist".to_string());
	};

	DB::new(id.clone()).set(body.key.clone(), body.value);

	ok(format!(
		"{} \"ok\": {}, \"db_id\": \"{}\" {}",
		'{', true, id, '}'
	))
}
