use std::fs::File;
use std::io::ErrorKind;

///# RESULT
/// ```rust
/// enum Result<T, E> {
///     Ok(T),
///     Err(E),
/// }
/// ```

#[test]
fn result_open_file() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

#[test]
fn result_open_create_file() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        }
    };
}