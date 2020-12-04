use std::path::PathBuf;
use serde_yaml::{Value, Mapping};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


#[derive(Debug, Clone)]
pub struct Config {
	pub path: PathBuf,
	mapping: Mapping
}

impl Config {
	pub fn new(path: PathBuf) -> Self {
		Self {
			path,
			mapping: Mapping::new()
		}
	}

	pub fn init(&mut self) {
		if !self.path.exists() {
			let path_parent = self.path.parent().unwrap();
			if !path_parent.exists() {
				std::fs::create_dir_all(path_parent).unwrap();
			}
			std::fs::write(&self.path, serde_yaml::to_string(&Mapping::new()).unwrap()).unwrap();
		}
	}

	pub fn load(&mut self) {
		self.mapping = serde_yaml::from_str(&std::fs::read_to_string(&self.path).unwrap()).unwrap();
	}

	pub fn save(&mut self) {
		std::fs::write(&self.path, serde_yaml::to_string(&self.mapping).unwrap()).unwrap();
	}

	pub fn set_default(&mut self, key: &str, value: Value){
		let key = Value::String(key.to_owned());
		if self.mapping.get(&key).is_none() {
			self.mapping.insert(key, value);
		}
	}

	pub fn get(&self, key: &str) -> Option<&Value> {
		let key = Value::String(key.to_owned());
		self.mapping.get(&key)
	}

	pub fn set(&mut self, key: &str, value: Value) {
		let key = Value::String(key.to_owned());
		self.mapping.insert(key, value);
	}
}
