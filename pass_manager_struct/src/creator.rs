use std::fs::create_dir; // function for creating a empty dir
use super::error::Error; // Taking to the scope trait Error

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
	pub fn create_directory(&self) -> Result<(),()> { // Creating a folder and file in local directory of program with names provide from user 

		let _string_with_folder = format!("../{}", &self.folder_name); // create a string with command of creating folder

		match create_dir(_string_with_folder){
			Ok(res) => Ok(()),
			Err(err) => Err(()),
		}// create directory with user name

	}
	// pub fn save_file_with_data(&self) {

	// }
}

impl Error for Creator {
	fn name_error(&self) -> String {
		format!("Diretory with that name already exist {:?}", &self.folder_name)
	}
}