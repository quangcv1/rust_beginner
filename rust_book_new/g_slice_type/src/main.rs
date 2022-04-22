fn main() {
    println!("Hello, SLICE!");
    /**
    # STRING SLICE
    **String slice** is a reference to part of a **String**
   */
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    /**
    # FIRST WORD
    */
    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("The first word is: {}", word);
    println!("The word is: {}", &s[1..2]);
    println!("String is: {}", s);
    let word = first_word(&s[0..6]); //s:&String vs s:&str

    /**
    # STRING LITERALS ARE SLICES
    */
    let s = "Hello, world";

    /**
    # STRING SLICES AS PARAMETERS
    - If we have a string slice, we can pass that directly.
    - If we have a String, we can pass a slice of the String or a reference to the String.
    - This flexibility takes advantage of ***deref coercion***
    ```rust
    fn first_word(s: &String) -> &str {
    ````
    */
    let test: &str = "Quang";

    /**
    # OTHER SLICE
    */
    let a = [1,2,3,4,5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2,3]);
}
//s:&str vs s:&String
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]
        }
    }
    &s[..]
}