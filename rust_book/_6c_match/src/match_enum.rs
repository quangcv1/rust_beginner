#[test]
fn match_enum() {
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),// enum in enum
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => { //with function return block
                println!("Lucky penny");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => { //with function return block including enum state
                println!("STate quarter from {:?}!", state);
                25
            },
        }
    }

    let coin = Coin::Penny;
    let value = value_in_cents(coin);
    println!("{}", value);

    let coin = Coin::Quarter(UsState::Alaska);
    println!("{}", value_in_cents(coin));
}




