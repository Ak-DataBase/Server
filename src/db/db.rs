use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{create_dir, read as read_file, write as write_file, File};
use std::path::PathBuf;

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
pub enum DBValue {
	Str(String),
	Bool(bool),
	Obj(HashMap<String, DBValue>),
	Num(f32),
	Null,
	Array(Vec<DBValue>)
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct DB {
	data: HashMap<String, DBValue>,
	pub file: PathBuf,
	pub id: String
}

#[allow(dead_code)]
impl DB {
	pub fn new(id: String) -> Self {
		let created_path = default_db_folder().join(id.clone());
		let file = created_path.clone();

		let mut ret = Self {
			data: HashMap::new(),
			file,
			id
		};

		ret.read();
		ret.write();
		ret
	}

	pub fn clear(&mut self) -> &mut Self {
		self.data.clear();
		self.write();
		self
	}

	pub fn get(&self, key: &'static str) -> Option<&DBValue> {
		self.data.get(key)
	}

	pub fn set(&mut self, key: &'static str, val: DBValue) -> Option<DBValue> {
		let ret = self.data.insert(key.to_string(), val);
		self.write();
		ret
	}

	fn write(&self) {
		let res = bincode::serialize(&self.data.clone()).unwrap();

		match write_file(self.file.clone(), res.clone()) {
			Ok(_) => (),
			Err(e) => panic!("Error writing DB file: {}", e)
		}
	}

	pub fn read(&mut self) {
		if !self.file.exists() {
			match File::create(self.file.clone()) {
				Ok(_) => (),
				Err(e) => panic!("Can't create DB file and file does not exist: {}", e)
			};
		}

		let contents = read_file(self.file.clone()).unwrap();

		if contents.len() == 0 {
			return;
		}

		let res: HashMap<String, DBValue> = bincode::deserialize(&contents[..]).unwrap();
		self.data = res;
	}
}
