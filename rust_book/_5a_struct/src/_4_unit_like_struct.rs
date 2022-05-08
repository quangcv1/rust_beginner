#[test]
fn unit_like_struct() {
    #[derive(Debug)]
    struct AlwaysEqual;

    let subject = AlwaysEqual;
    println!("Hello uni: {:#?}", subject);
}