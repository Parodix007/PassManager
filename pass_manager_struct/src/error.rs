pub #[derive(Debug)]
struct Error {
	error_msg: String,
}

impl Error {
	pub fn new(error_msg: String) -> Self {
		Self {
			error_msg
		}
	}
	pub fn name_error(&self){
		panic!("{:?}", &self.error_msg);
	}
}