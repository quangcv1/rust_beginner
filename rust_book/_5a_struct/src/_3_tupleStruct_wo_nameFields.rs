#[test]
fn tuple_struct() {
    #[derive(Debug)]
    struct Color(i32,i32,i32);
    #[derive(Debug)]
    struct Point(i32,i32,i32);

    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    println!("{:#?}, {:#?}", black,origin);
}