use crate::server::response404;
use crate::utils::request::{PostInfo, Request};

#[test]
fn raw_response() {
	assert_eq!(
		format!(
			"HTTP/1.1 {}{}\r\nContent-Type: {}; charset=UTF-8\r\n\r\n{}\r\n",
			404, " Not Found", "text/plain", "404"
		),
		response404().raw()
	)
}

#[test]
fn requests() {
	assert_eq!(
		Request {
			method: "GET".to_string(),
			post_info: None,
			sub_url: "/".to_string()
		},
		Request::new(
			"GET / HTTP/2\nHost: www.example.com\nUser-Agent: curl/7.54.0\nAccept: */*".to_string()
		)
	);

	assert_eq!(
		Request {
			method: "POST".to_string(),
			post_info: Some(PostInfo {
				body: "abc".to_string(),
				content_length: 3,
				content_type: "text/plain".to_string()
			}),
			sub_url: "/".to_string()
		},
		Request::new(
			r#"POST / HTTP/2
Host: www.example.com
User-Agent: curl/7.54.0
Accept: */*
Content-Type: text/plain
Content-Length: 3
abc"#
				.to_string()
		)
	);
}
