#[test]
fn regular_defer() {
    let x = 5;
    let y = &x;

    assert_eq!(x,5);
    assert_eq!(5,*y);
}