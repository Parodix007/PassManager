#[derive(Debug)]
struct Creator {
	file_name: String,
	os: String,
}

impl Creator {
	pub fn new(file_name: String, os: String) -> Self {
		Self{
			file_name,
			os
		}
	}
}