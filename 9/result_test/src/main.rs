use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
    read_username_from_file();
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    /*
    let mut f = File::open("hello.txt")?;
    f.read_to_string(&mut s)?;
    */
    // fs::read_to_string("hello.txt");
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
