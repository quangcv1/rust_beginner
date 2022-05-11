///# CONCATENATION WITH + OPERATOR
/// - s3 = s1 + &s2 Why not s3 = s1 + s2 or s3 = &s1 + &s2
/// + operator === ***add*** method
/// fn add(self, s:&str) -> String {
///
/// # CONCATENATION WITH FORMAT!
/// let s = format!("{}-{}-{}",s1,s2,s3);
#[test]
fn concatenation_string() {
    //with +
    let s1 = "Hello, ".to_string();
    let s2 = String::from("world");
    let s3 = s1 + &s2; //s1 has been moved here and no longer be used
    println!("{}", s3);

    let s1 = "tic".to_string();
    let s2 = "tac".to_string();
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);
}

#[test]
fn concatenation_format() {
    //with format
    let s1 = "tic".to_string();
    let s2 = "tac".to_string();
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1,s2,s3);
    println!("{}", s);
}
