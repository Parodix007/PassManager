use std::fs::create_dir; // Function for creating a empty dir
use super::error::Error; // Taking to the scope trait Error
use super::encrypt::Encrypt; // Struct for encrypt data
use std::io;
use std::fs::File; // File system
use std::io::prelude::*;
use serde_json; // For saving json to a file

#[derive(Debug)]
pub struct Creator {
	folder_name: String,
	file_name: String,
	os: String,
}

impl Creator {
	pub fn new(folder_name: String, file_name: String, os: String) -> Self {
		Self{
			folder_name,
			file_name,
			os,
		}
	}
	pub fn create_directory(&self) -> Result<(), io::Error> { // Creating a folder and file in local directory of program with names provide from user 

		let _string_with_folder = format!("../{}", &self.folder_name); // create a string with command of creating folder

		create_dir(_string_with_folder)?; // create directory with user name

		Ok(())
	}
	pub fn save_file_with_data(&self) -> bool{ // Function to save data to a file in directory that program create for a user
		let _encrypt = Encrypt::new(); // Instance of encrypt struct
		let mut _hash_with_data = serde_json::Map::new(); //Hash that contains pairs of webpage and password, it`s speciall hashmap from serde_json

		let mut number_of_iter = String::new();
		println!("Enter number of password you want to save");
		io::stdin().read_line(&mut number_of_iter).unwrap();
		let number_of_iter = number_of_iter.trim().parse().expect("Enter a number of passwords");

		for _ in 0..number_of_iter {
			let mut webpage_name = String::new();

			println!("---------------");
			println!("Enter a name of webpage for which you need to save password:");
			io::stdin().read_line(&mut webpage_name).unwrap();
			let _webpage_name = webpage_name.trim().to_string(); // Var with name of the webpage

			println!("---------------");
			let mut password_for_webpage = String::new();

			println!("Enter a password for webpage:");
			io::stdin().read_line(&mut password_for_webpage);
			
			let _password_for_webpage = password_for_webpage.trim().to_string(); // Var with password
			let _encrypted_pass = _encrypt.encrypt(_password_for_webpage); // Function call that encrypt password which user have entered

			if _hash_with_data.contains_key(&webpage_name.trim().to_string()) == false {
				_hash_with_data.insert(_webpage_name, serde_json::Value::String(_encrypted_pass));
			} else {
				panic!("Value already exist try again, sorry :(");
			}
		}

		let _file_path = format!("../{}/{}.json", self.folder_name, self.file_name);
		serde_json::to_writer(&File::create(_file_path).expect("Faild to create a fiel"), &_hash_with_data).expect("Faild to write to the file"); // Use a serde_json for a saving json obj to a file

		true 

	}
}

impl Error for Creator {
	fn name_error(&self) -> String {
		format!("Diretory with that name already exist {:?}", &self.folder_name)
	}
}