/**
# IN STRUCT DEFINITION with Same Data Type
*/
struct Point<T> {
    x: T,
    y: T,
}

/**
# IN STRUCT DEFINITION with Diff Data Type
*/
struct PointDiff<T> {
    x: T,
    y: U,
}
fn main() {
    let integer = Point {x:5, y:10};
    let float = Point {x: 1.0, y:4.0};
    let int_float = PointDiff{x: 5, y:4.0};
}
