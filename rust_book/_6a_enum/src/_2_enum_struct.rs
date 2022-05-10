#[test]
fn enum_with_struct_normal() {
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    #[derive(Debug)]
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("{:#?}, {:#?}", home, loopback)
}

#[test]
fn enum_only() {
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    println!("{:?}, {:?}", home, loopback)
}

#[test]
fn enum_only_enhance() {
    #[derive(Debug)]
    enum IpAddr {
        V4(u8,u8,u8,u8),
        V6(String),
    }

    let home = IpAddr::V4(127,0,0,1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("{:?}, {:?}", home, loopback)
}


