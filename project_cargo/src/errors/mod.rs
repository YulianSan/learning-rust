use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};
// three ways to handle with errors
// using functions expect, unwrap, ...
// using match
// using if let

pub fn example1() {
    panic!("Error");
}

pub fn example2() {
    let v: Vec<i32> = vec![1, 2, 3];

    println!("{}", v[99])
}

pub fn example3() {
    let f = File::open("test.txt");

    let file = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("test.txt") {
                Ok(file) => file,
                Err(e) => panic!("Error to create file: {}", e),
            },
            other_error => panic!("Problem opening the file {}", other_error),
        },
    };

    println!("file: {:?}", file.sync_data().expect("error"));
}

pub fn example4() {
    let file = File::open("test2.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            if let Err(error2) = File::create("test2.txt") {
                panic!("Erro when create file: {}", error2);
            } else {
                panic!("Created with success");
            }
        } else {
            panic!("Other error");
        }
    });
}

pub fn example5(file_name: &str) -> Result<String, io::Error> {
    let mut file = match File::open(file_name) {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    // return err or ok(content file)
    match file.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(err) => Err(err),
    }
}

// same behavior than example5 but smaller
pub fn example6(file_name: &str) -> Result<String, io::Error> {
    // the caracter ? pass error to function parent
    let mut file = File::open(file_name)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

pub fn example7(file_name: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(file_name)?.read_to_string(&mut s)?;

    Ok(s)
}
