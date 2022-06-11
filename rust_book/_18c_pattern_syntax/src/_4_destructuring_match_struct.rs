#[test]
fn destructuring_struct() {
    struct Point {
        x: i32,
        y: i32,
    }

    //destructuring a struct's fields into separate variables
    let p = Point{ x: 0, y: 7 };
    let Point{ x: a, y: b} = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    //using struct field shorthand
    let Point {x,y} = p;
    assert_eq!(0,x);
    assert_eq!(7,y);
}

#[test]
fn match_destructuring_struct() {
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };
    match p {
        Point{x, y: 0} => println!("on the x axis at {}", x),
        Point{x: 0, y} => println!("on the y axis at {}", y),
        Point{x, y} => println!("on neither axis: ({},{})", x,y),
    }
}