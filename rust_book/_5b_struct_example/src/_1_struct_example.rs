#[test]
fn struct_example() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    //normal func
    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }
    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels with origin {:#?}",
        area(&rec1),
        rec1
    );

}