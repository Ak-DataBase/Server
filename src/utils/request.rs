#[derive(Debug, Clone, PartialEq)]
pub struct PostInfo {
	pub body: String,
	pub content_length: i32,
	pub content_type: String
}

#[derive(Debug, Clone, PartialEq)]
pub struct Request {
	pub method: String,
	pub sub_url: String,
	pub post_info: Option<PostInfo>
}

impl Request {
	pub fn new(raw: String) -> Self {
		let lines: Vec<&str> = raw.split('\n').collect();
		let words_in_first_line: Vec<&str> = lines[0].split(' ').collect();
		let method = words_in_first_line[0].to_string();
		let sub_url = words_in_first_line[1].to_string();
		let post_info: Option<PostInfo>;

		if lines.len() > 5 {
			post_info = {
				let content_length_line_words: Vec<&str> = lines[5].split(' ').collect();
				let content_length_str = content_length_line_words[content_length_line_words.len() - 1];
				let content_type_line_words: Vec<&str> = lines[4].split(' ').collect();
				let content_type = content_type_line_words[1].trim().to_string();
				let content_length = content_length_str.trim().parse::<i32>().unwrap();
				let unfiltered_body = lines[lines.len() - 1];
				let body = unfiltered_body[..content_length as usize].to_string();

				Some(PostInfo {
					body,
					content_length,
					content_type
				})
			}
		} else {
			post_info = None
		};

		Self {
			method,
			sub_url,
			post_info
		}
	}
}
