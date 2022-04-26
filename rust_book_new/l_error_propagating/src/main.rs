#![allow(unused)]

use std::fs;

fn main() {
    use std::fs::File;
    use std::io::{self, Read};

    /** # NORMAL WAY WITH MATCH */

    fn read_username_from_file() -> Result<String, io::Error> {
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

    /** # ENHANCED WAY WITH ? */

    fn read_username_from_file_1() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }

    /** # SHORTEST WAY WITH ? */

    fn read_username_from_file_2() -> Result<String, io::Error> {
        let mut s = String::new();

        File::open("hello.txt")?.read_to_string(&mut s)?;

        Ok(s)
    }

    /** # SHORTEST WAY */

    fn read_username_from_file_3() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }
}
