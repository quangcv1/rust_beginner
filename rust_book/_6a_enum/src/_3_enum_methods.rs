#[test]
fn enum_methods() {
    enum Message {
        Quit, //no data
        Move {x: i32, y: i32}, //like struct
        Write(String), //String
        ChangeColor(i32,i32,i32), //tuple
    }

    impl Message {
        fn call(&self) {
            println!("call is invoked");
        }
    }

    let m = Message::Write(String::from("Hello"));
    m.call();
}

