fn main() {
    println!("Hello, world!");
}

#[test]
fn dereference_regular() {
    let x = 5;
    let y = &x;

    assert_eq!(5,x);
    assert_eq!(5, *y);
}

#[test]
fn dereference_box() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5,x);
    assert_eq!(5, *y);
}