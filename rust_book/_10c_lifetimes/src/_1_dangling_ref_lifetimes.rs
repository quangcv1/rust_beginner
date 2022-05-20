///# LIFETIME ANNOTATION SYNTAX
/// &i32 - a reference
/// &'a i32 - a reference with an explicit lifetime
/// &'a mut i32 - a mutable reference with an explicit lifetime
/// <'a> lifetime annotations in function signatures
#[test]
fn dangling_ref_lifetimes() {
    //the returned ref will be valid as long as both the paras are
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else { y }
    }

    let string1 = "abcd ab".to_string();
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}