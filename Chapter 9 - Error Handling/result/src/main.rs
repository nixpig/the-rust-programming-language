use std::fs::{self, File};
use std::io::{Error, ErrorKind, Read, Write};

fn main() {
    let greeting_file_result = File::open("hello1.txt");

    let mut greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello1.txt") {
                Ok(file) => file,
                Err(e) => panic!("Problem creating file: {:?}", e),
            },

            other_error => panic!("Some other error occurred: {:?}", other_error),
        },
    };

    let message = "Hello, world!";

    match greeting_file.write_all(message.as_bytes()) {
        Ok(_) => println!("Successfully wrote to file."),
        Err(e) => panic!("Error writing to file: {:?}", e),
    }

    println!("----------");

    let mut greeting_file_handle = File::open("hello2.txt").unwrap_or_else(|e| {
        if e.kind() == ErrorKind::NotFound {
            let greeting_file = File::create("hello2.txt").unwrap_or_else(|e| {
                panic!("Could not create file: {:?}", e);
            });

            return greeting_file;
        } else {
            panic!("Could not open file: {:?}", e);
        }
    });

    greeting_file_handle
        .write_all("Hello 2".as_bytes())
        .unwrap_or_else(|e| {
            panic!("Could not write to file: {:?}", e);
        });

    println!("-----------");

    // let mut greeting_file_result = File::open("hello3.txt").expect("Should exist!");
    // println!("file: {:?}", greeting_file_result);

    let username = read_username_from_file_explicit_error_propagation("username.txt")
        .unwrap_or_else(|e| {
            panic!("Couldn't read username from file: {:?}", e);
        });

    println!("Username is: {username}");

    println!("----------------");

    let username = read_username_from_file_implicit_error_propagation("username2.txt")
        .expect("Username file to exist");
    println!("Second username is: {username}");
}

fn read_username_from_file_explicit_error_propagation(file_path: &str) -> Result<String, Error> {
    let username_file_result = File::open(file_path);

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    return match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    };
}

fn read_username_from_file_implicit_error_propagation(file_path: &str) -> Result<String, Error> {
    let mut username = String::new();

    File::open(file_path)?.read_to_string(&mut username)?;

    return Ok(username);
}

fn read_username_from_file_and_return(file_path: &str) -> Result<String, Error> {
    return fs::read_to_string(file_path);
}
