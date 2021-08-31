use super::db_struct::DB;
use super::utils::{bad_request, ok, JSON};
use crate::utils::{request::Request, response::Response};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GetBody {
	db_id: String,
	key: String
}

pub fn get(req: Request) -> Response {
	if req.method != "GET" {
		return bad_request(format!("Expected method GET, found {}", req.method));
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

	let body = match serde_json::from_str::<GetBody>(&raw_body) {
		Ok(x) => x,
		Err(e) => return bad_request(format!("Error parsing JSON: {}", e))
	};

	let id = body.db_id;

	if !DB::exists(id.clone()) {
		return bad_request("Specified DB id does not exist".to_string());
	};

	let val = match DB::new(id.clone()).get(body.key.clone()) {
		Some(x) => x.to_string(),
		None => {
			return bad_request(format!(
				"No value with key \"{}\" found in DB with id \"{}\"",
				body.key, id
			))
		}
	};

	ok(format!(
		"{} \"val\": {}, \"db_id\": \"{}\" {}",
		'{', val, id, '}'
	))
}
