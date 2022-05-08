///# REFERENCE'S SCOPE
///- STarts from where it is introduced and continues through the last time that reference is used.
///- (r1,r2) scope and (r3) scope don't overlap so below code is allowed
#[test]
fn ref_scope() {
    let mut s = String::from("hello");

    let r1 = &s; //no problem
    let r2 = &s; //no problem
    println!("{} and {}", r1,r2);
    //variables r1 and r2 will not be used after this point
    let r3 = &mut s;
    //the last usage of the immutable references ***println!***, occurs before the mutable ref is introduce.
    println!("{}", r3);
}