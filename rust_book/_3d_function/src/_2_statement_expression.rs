///
/// # STATEMENT
/// # Description:
/// - Statement do not return values
/// - Can't assign a ***let*** statement to another variable
#[test]
fn statement_err() {
    let y = 6; //statement
    //let x = (let y = 6); //error
}

///# EXPRESSION
#[test]
fn expression() {
    let y =
    //this expression
        {
            let x = 3;
            x + 1 //return value
        };
    println!("The value of y is {}", y);
}