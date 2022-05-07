#[test]
fn loop_return_value() {
    let mut counter =0;

    let result = loop {
        counter +=1;

        if counter == 10 {
            break counter * 2; //this is special
        }
    };

    println!("The result is {}", result);
}