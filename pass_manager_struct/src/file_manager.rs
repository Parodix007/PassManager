use std::fs::File;
use std::fs::OpenOptions; // For open file in write mode only
use serde_json;
use serde_json::Map;
use std::io;
use std::io::prelude::*;

use super::encrypt::Encrypt;
use std::collections::HashMap;

use termion::clear;
#[derive(Debug)]
pub struct FileContent {
	file_name: String,
	folder_name: String,
}

impl FileContent {
	pub fn new(file_name: String, folder_name: String) -> Self {
		Self {
			file_name,
			folder_name,
		}
	} 
	pub fn show_content(&self) -> String{
		let _decrypt = Encrypt::new();
		let mut webpage = String::new();
		println!("{}", clear::All);
		println!("---------------");
		println!("Enter a name of webpage for which you need password: ");
		io::stdin().read_line(&mut webpage).unwrap();
		let _webpage = webpage.trim().to_string();

		let _file_path = format!("../{}/{}.json", self.folder_name, self.file_name);

		let _file = File::open(_file_path).expect("Faild to open a file");

		let _content: HashMap<String, String> = serde_json::from_reader(&_file).unwrap(); // Use a serde_json to read content of a file in json format

		let _value_of_key = _content.get(&_webpage).unwrap();

		let _decrypted_value = _decrypt.decrypt(_value_of_key.to_string());

		_decrypted_value
	}
	pub fn update_file(&self) -> bool{
		let _encrypt = Encrypt::new();
		let mut option = String::new();
		let mut new_webpage = String::new();
		let mut new_password = String::new();

		let _file_path = format!("../{}/{}.json", self.folder_name, self.file_name);

		let _file = File::open(&_file_path).expect("Faild to open a file");
		let mut _content: Map<String, serde_json::Value> = serde_json::from_reader(&_file).unwrap();
		drop(_file); // Close connection with the file

		println!("{}", clear::All);
		println!("---------------");
		println!("1.Change exisiting password\n2.Add new password to existing file");
		io::stdin().read_line(&mut option).unwrap();
		println!("---------------");
		let _option: i8 = option.trim().parse().unwrap();

		if _option == 1 {
			println!("Enter a name of webpage where is new password");
			io::stdin().read_line(&mut new_webpage).unwrap();
			let _new_webpage = new_webpage.trim();
			println!("---------------");
			println!("Enter a new password");
			io::stdin().read_line(&mut new_password).unwrap();
			let _new_password = _encrypt.encrypt(new_password.trim().to_string());

			_content[_new_webpage] = serde_json::Value::String(_new_password.to_string());

			let _update_file = OpenOptions::new()
											.write(true)
											.read(false)
											.create(false)
											.open(_file_path).unwrap();

			serde_json::to_writer(&_update_file, &_content).expect("Faild to update fail");
		} else if _option == 2 {
			println!("Enter a name of new webpage");
			io::stdin().read_line(&mut new_webpage).unwrap();
			let _new_webpage = new_webpage.trim();
			println!("---------------");
			println!("Enter a new password");
			io::stdin().read_line(&mut new_password).unwrap();
			let _new_password = _encrypt.encrypt(new_password.trim().to_string());

			_content.insert(_new_webpage.to_string(), serde_json::Value::String(_new_password.to_string()));

			let _update_file = OpenOptions::new()
											.write(true)
											.read(false)
											.create(false)
											.open(_file_path).unwrap();

			serde_json::to_writer(&_update_file, &_content).expect("Faild to update fail");
		} else {
			panic!("You provide a wronge option");
		}
		true 
	}
}