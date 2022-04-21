fn main() {

    /**
    Immutable & mutable variable
    */

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    /**
    Constants:
    - Not allowed to use mut
    - Declare using const keyword instead of let
    - ALL uppercase with underscores between words
    */
    const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;

    /**
    SHADOWING:
    - using for transformations on a value but have the variable be immutable after these transformations have been completed
    - can change the type of the value but reuse the same name.
    */
    let s = 5;
    let s = s+ 1;
    {
        let s = s* 2;
        println!("The value of x in the inner scope is: {}", s); //12
    }
    println!("The value of x is: {}", s); //6

    //Change the type
    let spaces = "    ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);
    //change the type with mut then error
    let mut spaces_new = "         ";
    //spaces_new = spaces_new.len();
}
