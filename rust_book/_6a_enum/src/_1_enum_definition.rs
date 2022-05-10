#[test]
fn enum_definition() {
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("{:?}, {:?}", four, six);

    let four_bk = four;
    println!("{:?}",four_bk);
    //println!("{:?}",four);
}