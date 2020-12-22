use std::env;
use std::io;
fn main() {
    let _os = env::consts::OS;
    let mut user_option = String::new();

    println!("---------------");
    println!("Menu:\n1.Create file with passwords\n2.Read passwords from a existing file");
    println!("---------------");
    println!("Enter a option:");
    io::stdin().read_line(&mut user_option).expect("Enter option");

    println!("{:?}", user_option.trim());
}
