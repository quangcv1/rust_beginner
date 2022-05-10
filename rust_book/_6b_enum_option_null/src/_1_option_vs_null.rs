#[test]
fn option_vs_null() {
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    println!("{:?}, {:?}, {:?}", some_number, some_string, absent_number);

}

#[test]
fn option_error() {
    let x: i8 = 5;
    let y = Some(5);

    // let sum = x + y; //Error here
    // println!("{:?}", y);
}