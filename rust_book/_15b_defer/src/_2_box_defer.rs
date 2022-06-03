#[test]
fn box_defer() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5,x);
    assert_eq!(5,*y);
}