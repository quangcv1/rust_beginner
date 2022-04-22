#![allow(unused)]
fn main() {
    /**
    # DEFINE ENUM
    */
    enum IpAddrKind {
        V4,
        V6,
    }

    /**
    # ENUM WITH VALUE
    */
    enum IpAddrValue {
        V4(String),
        V6(String),
    }

    /**
    # ENUM WITH VALUE CONT
    */
    enum IpAddrValueCon {
        V4(u8,u8,u8,u8),
        V6(String),
    }

    /**
    # DEFINE METHODS
    */
    impl IpAddrValueCon {
        fn call(&self) -> &str {
            "Quang"
        }
    }
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

    let homeValue = IpAddrValue::V4(String::from("127.0.0.1"));
    let loopbackValue = IpAddrValue::V6(String::from("::1"));

    let homeValCon = IpAddrValueCon::V4(127,0,0,1);
    let loopbackCon = IpAddrValueCon::V6(String::from("::1"));
    let test = IpAddrValueCon::V6(String::from("Hang"));
    println!("{}",test.call());

}


