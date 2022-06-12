///# @Bindings
/// - create a var that holds a value at the same time we're testing that value
/// to see wethere it matches a pattern.

#[test]
fn binding() {
    enum Message {
        Hello {id: i32},
    }

    let msg = Message::Hello {id: 13};

    match msg {
        Message::Hello {id: id_variable @ 3..=7,} => {
            println!("Found an id in range: {}", id_variable)
        },
        Message::Hello {id: 10..=12} => {
            println!("Found an id in another range 10 - 12");
        },
        Message::Hello {id: id_var} => {gyiuhiasdxrthe[];\=]0[iopk]
            println!("Found some other id: {}", id_var);
        }
    }
}