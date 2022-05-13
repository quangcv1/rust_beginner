///# IN STRUCT DEFINITIONS
/// - We can define structs to use a ***generic type parameter*** in one or more fields using ***<>***
#[test]
fn in_struct() {
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
        //x and y are both that same type, whatever that type may be.
    }

    let integer = Point {x: 5, y:10};
    let float = Point {x:1.0, y:4.0};
    println!("{:?}, {:?}", integer, float);
}

#[test]
fn in_struct_with_different_type() {
    #[derive(Debug)]
    struct Point_new<T, U> {
        x: T,
        y: U,
    }

    let both_integer = Point_new{x:5, y:10};
    let both_float = Point_new{x:1.0, y:4.0};
    let integer_and_float = Point_new{x:5, y:4.0};
    println!("{:?}, {:?}, {:?}", both_integer, both_float, integer_and_float);
}
