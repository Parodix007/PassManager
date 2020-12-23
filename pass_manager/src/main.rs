use std::env;
use std::io;

use pass_manager_struct::creator::Creator;
use pass_manager_struct::file_manager::File;

fn main() {
    let _os = String::from(env::consts::OS); // Create a var with name of user os
    let mut user_option = String::new();

    // Display simple menu
    println!("---------------");
    println!("Menu:\n1.Create file with passwords\n2.Read passwords from a existing file");
    println!("---------------");
    println!("Enter a option:");
    io::stdin().read_line(&mut user_option).expect("Enter option"); // Taking option from user

    let _user_option_parsed: i8 = user_option.trim().parse().expect("Error while parsing"); // parsing option number

    if _user_option_parsed == 1 { // running program of option 1

    	let mut folder_name = String::new(); // Taking name of folder from user
    	println!("Enter a name for you`r folder:");
    	io::stdin().read_line(&mut folder_name).expect("Enter a folder name");

    	let mut file_name = String::new(); // Taking name of file from user
    	println!("Enter a name for you`r file:");
    	io::stdin().read_line(&mut file_name).expect("Enter a file name");

    	let _creator = Creator::new(folder_name.trim().to_string(), file_name.trim().to_string(), _os); // Create a instance of Creator with value provide from user
    	_creator.create_directory(); // creating a empty dir which is going to contain a file with password
    	
    } else if _user_option_parsed == 2 { // running program of option 2
    	println!("Choose secound option");
    }else { // return with error
    	panic!("Some error occur probably you provide a wrong number");
    }
}
