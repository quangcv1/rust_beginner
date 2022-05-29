///# CLOSURES: ***move*** keyword
///- to force the closure to take ownership of the values it uses in the env
///- use ***move*** keyword before the parameter list
///- Useful when passing a closure to a new thread to move the data so it's owned by the new thread
#[test]
fn closure_move() {
    let x = vec![1,2,3];//vectors instead of int because
    //int can be copied rather than moved
    let equal_to_x = move |z| z == x;
    // println!("can't use x here: {:?}", x);
    let y = vec![1,2,3];
    assert!(equal_to_x(y));
}