#[test]
fn variable_scope() {
    let s = "Quang";

    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward
        println!("{}", s);
        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
    println!("{}", s);

    let mut str = "Quang";
    str = "Hang";
    println!("{}", str);
}



