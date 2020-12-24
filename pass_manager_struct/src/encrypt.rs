use magic_crypt::MagicCrypt; // Use to create new instance of encrypt thing
use magic_crypt::SecureBit; // Type of encrypt
use magic_crypt::MagicCryptTrait; // Trait for functions

#[derive(Debug)]
pub struct Encrypt { // Struct that will encrypt and decrypt password
	key: MagicCrypt,
}

impl Encrypt {
	pub fn new() -> Self {
		Self{
			key: MagicCrypt::new("crypt", SecureBit::Bit128, Some("crypt")),
		}
	}
	pub fn encrypt(&self, password: String) -> String { // Encrypt password 
		self.key.encrypt_str_to_base64(password)
	}
	pub fn decrypt(&self, encrypted_password:String) -> String { // Decrypt password
		self.key.decrypt_base64_to_string(encrypted_password).unwrap()
	}
}