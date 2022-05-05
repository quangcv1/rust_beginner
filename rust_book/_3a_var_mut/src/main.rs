fn main() {
    println!("Hello, world!");
}

#[test]
fn assign_twice_to_immut_var() {
    let x = 5;
    println!("The value of x is: {}", x);
    //x = 6; //Error here
    println!("The value of x is: {}", x);
}

#[test]
fn assign_twice_to_mut_var() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6; //Error here
    println!("The value of x is: {}", x);
}

#[test]
fn constants() {
    const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;
    println!("{}", THREE_HOURS_IN_SECONDS);
}

#[test]
/// # SHADOWING USE CASE
/// - Performing transformations on a value with immutable variable
fn shadowing() {
    let x = 5;
    let x = x + 1; //shadowing
    {
        let x = x*2; //inner shadowing
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);
}

#[test]
fn shadowing_pros() {
    //change type with reusing the same name
    let spaces = "  ";
    let spaces = spaces.len();
    println!("{}", spaces);
}

#[test]
fn mut_pros() {
    let mut spaces = "   ";
    //spaces = spaces.len(); //Error here
}