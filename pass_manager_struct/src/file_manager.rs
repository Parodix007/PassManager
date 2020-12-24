use std::fs::File;
use serde_json;

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
}