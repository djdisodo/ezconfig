use std::path::PathBuf;
use serde_yaml::Value;
use std::ops::{Deref, DerefMut};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


#[derive(Debug, Clone)]
pub struct Config {
	pub path: String,
	value: Value
}

impl Config {
	pub const fn new(path: String) -> Self {
		Self {
			path,
			value: Value::Null
		}
	}

	pub fn init(&mut self) -> PathBuf {
		let path = PathBuf::from(&self.path);
		if !path.exists() {
			let path_parent = path.parent().unwrap();
			if !path_parent.exists() {
				std::fs::create_dir_all(path_parent).unwrap();
			}
			std::fs::write(&path, serde_yaml::to_string(&Value::Null).unwrap()).unwrap();
		}
		path
	}

	pub fn load(&mut self) {
		let path = self.init();
		self.value = serde_yaml::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
	}

	pub fn save(&mut self) {
		let path = self.init();
		std::fs::write(&path, serde_yaml::to_string(&self.value).unwrap()).unwrap();
	}
}

impl Deref for Config {
	type Target = Value;

	fn deref(&self) -> &Self::Target {
		&self.value
	}
}

impl DerefMut for Config {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.value
	}
}