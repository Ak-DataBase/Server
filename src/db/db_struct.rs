use serde::{Deserialize, Serialize};
use serde_json::value::Value;
use std::{
	collections::HashMap,
	fs::{create_dir, read, read_to_string, File},
	io::Write,
	path::PathBuf
};

pub fn default_db_folder() -> PathBuf {
	let home = match dirs::home_dir() {
		Some(x) => x.join(".akdb"),
		None => panic!("Unable to get home dir")
	};

	if !home.exists() {
		match create_dir(home.clone()) {
			Ok(_) => (),
			Err(e) => panic!("Can't create DB folder and folder doesn't exist: {}", e)
		};
	}

	home
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct DB {
	pub data: HashMap<String, Value>,
	pub file: PathBuf,
	pub id: String
}

#[allow(dead_code)]
impl DB {
	pub fn new(id: String) -> Self {
		let file = default_db_folder().join(id.clone());

		let mut ret = Self {
			data: HashMap::new(),
			file,
			id
		};

		ret.create_file();
		ret.read();
		ret.write();
		ret
	}

	pub fn exists(id: String) -> bool {
		default_db_folder().join(id).exists()
	}

	pub fn clear(&mut self) -> &mut Self {
		self.data.clear();
		self.write();
		self
	}

	pub fn get(&self, key: String) -> Option<&Value> {
		self.data.get(&key)
	}

	pub fn set(&mut self, key: String, val: Value) -> &mut Self {
		self.data.insert(key, val);
		self.write();
		self
	}

	fn write(&self) {
		let data = &self.data.clone();
		let data_str = serde_json::to_string(data).unwrap();
		let res = data_str.as_bytes();

		match File::create(self.file.clone()).unwrap().write(res) {
			Ok(_) => (),
			Err(e) => panic!("Error writing DB file: {}", e)
		}
	}

	pub fn read(&mut self) -> HashMap<String, Value> {
		let contents = read(self.file.clone()).unwrap();
		let contents_str = read_to_string(self.file.clone()).unwrap();

		if contents_str.trim().is_empty() {
			return HashMap::new();
		}

		let data = String::from_utf8(contents).unwrap();

		let res: HashMap<String, Value> = serde_json::from_str(&data[..]).unwrap();
		self.data = res.clone();
		res
	}

	fn create_file(&self) {
		if !self.file.exists() {
			match File::create(self.file.clone()) {
				Ok(_) => (),
				Err(e) => panic!("Can't create DB file and file does not exist: {}", e)
			};
		}
	}
}
