use std::fs::File;
use std::io::ErrorKind;

#[test]
fn result_alternative_1() {
    let f = File::open("hello.txt")
        .unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Problem creating the file: {:?}", error);
                })
            } else {
                panic!("Problem opening the file: {:?}", error);
            }
        });
}

#[test]
fn result_alternative_2() {
    //unwrap:
    //1. Ok => return the value inside the Ok
    //2. Err => call the panic!
    let f = File::open("hello.txt").unwrap();
}

#[test]
fn result_alternative_3() {
    //expect:
    //1. Ok => return the value inside the Ok
    //2. Err => providing error messages
    let f = File::open("hello.txt")
        .expect("Failed to open hello.txt");
}