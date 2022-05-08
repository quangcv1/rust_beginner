///# IF EXPRESSION
#[test]
fn control_if() {
    let number = 6;
    if number%4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4,3, or 2");
    }
}

///# IF IN A LET STATEMENT
#[test]
fn control_if_let() {
    let condition = true;
    let number = if condition {5} else { 6 };

    println!("The value of number is: {}", number);
}

#[test]
fn control_if_let_err() {
    // let condition = true;
    // let number = if condition {5} else { "six" };
    // println!("The value of number is: {}", number);
}