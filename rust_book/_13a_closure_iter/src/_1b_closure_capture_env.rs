///# CLOSURES: Capturing the Environment
/// - it uses memory to store the values for use in the closure body
/// - cause to use of memory is overhead
///
///# CLOSURES: Capturing the Env in 3 ways
/// - taking ownership | FnOnce: take ownership and move them into the closure when it is defined
/// The Once part represents the fact that the closure can't take ownership of the same var more than once
/// - borrowing mutably | FnMut: can change the env because it mutably borrow values
/// - borrowing immutably | Fn: borrows values from the env immutably.
#[test]
fn closure_capture_env() {
    let x = 4;
    let equal_to_x = |z| z == x;//capture x here
    //=> so equal_to_x has the Fn trait
    let y = 4;
    assert!(equal_to_x(y));
}

#[test]
fn func_capture_env_err() {
    // let x = 4;
    // fn equal_to_x(z: i32) -> bool {
    //     z == x
    // }
    // let y = 4;
    // assert!(equal_to_x(y));
}