// Error handling
// panic
use std::fs::File;
use std::io::{self, ErrorKind, Read};
fn main() {
    // panic!("THis is a basic panic signal")

    // use a panic backtrace
    // let vector = vec![1, 2, 4];
    // println!("is it runtime?");
    // vector[30];

    // handling errors gracefully
    // let f = File::open("hello.txt");

    // let _f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error)
    //         }
    //     },
    // };

    // use except trait
    // let _f = File::open("hello.txt").expect("Failed to open hello.txt");

    // general error getting from function and handling
    // general_read_username_from_file().expect("Something bad happend");
    // shorthand error
    // short_read_username_from_file().expect("something bad happend in short function");
}

// use short hand error handling which uses From trait
fn _short_read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// general error returning from a function
fn _general_read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
