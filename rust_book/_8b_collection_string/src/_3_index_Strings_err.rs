///# STRING INDEX ERROR
/// Error because the way that Rust stores strings in memory
/// Ex: "Hola" => len 4 => 4 bytes long => each take 1 byte when encoded in UTF-8
/// Ex: "Здравствуйте" => len not 12 but 24 => each take 2 bytes when encoded in UTF-8
#[test]
fn index_string_err() {
    // let s1 = String::from("hello");
    // let h = s1[0]; //return byte then it should be 104
    //because as_bytes as below is [104,101,108,111]

    //convert as_bytes
    let s1 = String::from("hello");
    let s1_bytes = s1.as_bytes();
    println!("{:?}", s1_bytes);

    // let hello = "Здравствуйте";
    // let anwser = &hello[0];

}

#[test]
fn slice_string_err() {
    let hello = "Здравствуйте";
    let hello_bytes = hello.as_bytes();
    println!("{:?}", hello_bytes); //=> each char is 3 bytes
    let s = &hello[..2]; //error if  [0..1] , [0..3], [0..5]
    println!("{}", s);
}