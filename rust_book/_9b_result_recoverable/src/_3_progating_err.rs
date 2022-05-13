use std::fs::File;
use std::{fs, io};
use std::io::Read;

#[test]
fn propagating_err() {
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }

    fn read_username_from_file_enhanced() -> Result<String, io::Error> {
        let mut s = String::new();
        File::open("hello.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }

    fn read_username_from_file_3() -> Result<String,io::Error> {
        fs::read_to_string("hello.txt")
        //read_to_string: opens the file
        //, creates a new String
        //, reads the contents of the file
        //, puts the contents into that String
        //, returns it
        //Note: fs::read_to_string doesn't give us the opportunity to explain
        //all the error handling.
    }

    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }

    let text = "\
    nhi";
    let t = last_char_of_first_line(&text);
    println!("{:?}", t);
}