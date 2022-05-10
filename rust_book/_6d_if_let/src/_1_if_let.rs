#[test]
fn if_let() {
    let config_max = Some(3u8);
    //match logic
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    //if let logic
    if let Some(max) = config_max {
        println!("The maximum: {}", max)
    }
}

#[test]
fn if_let_else() {
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }
    #[derive(Debug)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),// enum in enum
    }

    let mut count = 0;
    //normal
    let coin = Coin::Penny;
    match coin {
        Coin::Quarter(state) => println!("State Quater from {:?}", state),
        _=> count +=1,
    }
    println!("{}", count );
    //println!("{:?}", coin); //error: value borrowed here after partial move.
    //Partial move occurs because value has type `UsState`, which does not implement the `Copy` trait

    //if let
    let coin_b = Coin::Quarter(UsState::Alaska);
    if let Coin::Quarter(state) = coin_b {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }

    //println!("{:?}", coin_b); //error: value borrowed here after partial move.
    //Partial move occurs because value has type `UsState`, which does not implement the `Copy` trait
}