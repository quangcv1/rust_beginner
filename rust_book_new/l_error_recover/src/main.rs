use std::fs::File;

fn main() {
    /**
    # RESULT
    ```rust
    #![allow(unused)]
    fn main() {
        enum Result<T, E> {
            Ok(T),
            Err(E),
        }
    }
        ```
    */
    //let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };
    /** # OR */
    // let f = File::open("hello.txt");
    // if let Err(error) = f {
    //     panic!("Problem opening the file 2: {:?}", error);
    // }
    /** # UNWRAP */
    // let f = File::open("hello.txt").unwrap();
    /** # EXPECT */
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
