// result enum
use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    open_file_2();
}

fn open_file_1() {
    let f = File::open("hello.text");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.text") {
                Ok(file) => file,
                Err(error) => panic!("Faile create file: {:?}", error),
            },
            other_error => panic!("Faile open file: {:?}", other_error),
        },
    };
}

fn open_file_2() {
    let f = File::open("hello.text").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.text").unwrap()
        } else {
            panic!("Faile create file: {:?}", error);
        }
    });
}

fn read_username_from_file_1() -> Result<String, io::Error> {
    let mut f = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(error) => Err(error),
    }
}

fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut s = String::new();
    let mut f = File::open("hello.txt")?;

    f.read_to_string(&mut s)?;

    Ok(s)
}

fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut s = String::new();
    let mut f = File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
