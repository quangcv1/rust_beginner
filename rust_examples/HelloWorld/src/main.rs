mod lib;

fn main() {
    println!("Hello, world!");
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {}", x, y);
}

#[test]
fn match_guards() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {}", x, y);
}

#[test]
fn match_guards_or () {
    let x = 4;
    let y = true;
    match x {
        4|5|6 if y => println!("yes"),
        _ => println!("no"),
    }
}

#[test]
fn match_guards_binding() {
    enum Message {
        Hello {id: i32},
    }

    let msg = Message::Hello {id: 14};

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello {
            id: 10..=12
        } => {
            println!("Found an id in another range");
        },
        Message::Hello {id} => println!("Found some other id: {}", id),
    }
}