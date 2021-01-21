# PassManager
Application for password management written in Rust. CLI

# Main Goal
* This appliaction you can use as a simple password manager. Nothing fancy about it, it is a simple project just for fun but it can be handy when you forgot some password. I do not know if it can proof you from a some kind of data leak from your computer I hope so but as I said I am not sure.

# Enviroment
* I am using here only rust and rust crates. Everything is in `Cargo.toml`.  

# Note about error
* If you face an error on windows, you have to delete termion from `Cargo.toml` and every `println!("{}", clear::All);` from the source code.
