use std::env;
use std::io;

use pass_manager_struct::creator::Creator;
use pass_manager_struct::file_manager::FileContent;
use pass_manager_struct::error::Error;

use termion::clear;
fn main() {
    let _os = String::from(env::consts::OS); // Create a var with name of user os
    let mut user_option = String::new();

    // Display simple menu
    println!("{}", clear::All);
    println!("---------------");
    println!("Menu:\n1.Create file with passwords\n2.Read password from an existing file");
    println!("---------------");
    println!("Enter a option:");
    io::stdin().read_line(&mut user_option).unwrap(); // Taking option from user

    let _user_option_parsed: i8 = user_option.trim().parse().expect("Error while parsing"); // parsing option number

    println!("---------------");
    println!("DIRECTORY CREATING");
    let mut folder_name = String::new(); // Taking name of folder from user
    println!("Enter a name for your folder:");
    io::stdin().read_line(&mut folder_name).expect("Enter a folder name");
    println!("---------------");

    println!("---------------");
    println!("FILE CREATING");
    let mut file_name = String::new(); // Taking name of file from user
    println!("Enter a name for you`r file:");
    io::stdin().read_line(&mut file_name).expect("Enter a file name");
    println!("---------------");

    if _user_option_parsed == 1 { // running program of option 1
    	let _creator = Creator::new(folder_name.trim().to_string(), file_name.trim().to_string(), _os); // Create a instance of Creator with value provide from user

    	match _creator.create_directory() {
    		Ok(()) => println!("Directory created without any problem have fun"),
    		Err(err) => panic!(_creator.name_error()),
    	}; // creating a empty dir which is going to contain a file with password or when the directory already exist return error

        if _creator.save_file_with_data() == true {
            println!("---------------");
            println!("Everything is done thanks for using my system");
        }
    	
    } else if _user_option_parsed == 2 { // running program of option 2
    	let _file_content = FileContent::new(file_name.trim().to_string(), folder_name.trim().to_string());
        let _password = _file_content.show_content();
        println!("---------------");
        println!("Your password: {:?}", _password);
        println!("---------------");
    }else { // return with error
    	panic!("Some error occur probably you provide a wrong number");
    }
}
