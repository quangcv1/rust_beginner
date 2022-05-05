use std::io;

#[test]
fn array() {
    let a = [1,2,3,4,5];
    let a: [i32;5] = [1,2,3,4,5];
    println!("{:?}",a);
}

#[test]
fn array_same_value() {
    let a = [3;5];
    println!("{:?}",a);
}

#[test]
fn access_array() {
    let a = [1,2,3,4,5];

    let first = a[0];
    let second = a[1];
    assert_eq!(vec![first,second], vec![1,2])
}

/// # COMMAND LINE WITH TEST
/// cargo test --package _3c_compound
// _types --bin _3c_compound_types array_type::invalid_array_access -- --exact
#[test]
fn invalid_array_access() {
    let a = [1,2,3,4,5];

    println!("Pls enter an array index.");
    let mut index = String::new();
    io::stdin().read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim() //trim space of string
        .parse() //parse to desired type
        .expect("Index enter was not a number");

    let element = a[index];
    println!("index: {} value: {}", index, element);
}