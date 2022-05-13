#[test]
fn panic_direct() {
    panic!("crash and burn");
}

///$ RUST_BACKTRACE=1 cargo run
#[test]
fn panic_indirect() {
    let v = vec![1,2,3];
    v[99];
}