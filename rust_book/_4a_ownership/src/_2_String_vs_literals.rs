/// # String vs literals
/// - ***String*** can be mutated
/// - Literals cannot
/// ## String type can do:
/// - The mem must be requested from the mem allocator at runtime
/// => ***String::from*** do it
/// - Returning this mem to the allocator when we're done with our ***String***
/// => automatically returned once the variable that owns it goes out of scope
/// => Rust call ***drop*** function
#[test]
fn string_type() {
    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }                                  // this scope is now over, and s is no
    // longer valid
}
