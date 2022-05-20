///# THE STATIC LIFETIME
/// - denotes that affected reference can live for the entire duration of the program
/// - is stored directly in the program's binary, which is always available
/// - most of the time, an error message suggesting the 'static lifetime
///

#[test]
fn string_literals_lifetime() {
    let s: &'static str = "I have a static lifetime";
}