fn main() {
    println!("Functions!");
    another_function();
    another_fun_para(5);
    /**
    STATEMENTS AND EXPRESSIONS
    - STATEMENTS: statement does not return values
    - EXPRESSIONS: calling a function is an expression, calling a macro is an expression, a new scope block with curly brackets is an expression
    */
    //let x = (let y = 6);
    let x = five();
    println!("The value of x is: {}", x);
    let x = plus_one(5);
    println!("The value of x is: {}", x);
}
/**
FUNCTION
*/
fn another_function() {
    println!("Another function.");
}

/**
FUNCTION - PARAMETERS
*/

fn another_fun_para(x: u32) {
    println!("The value of x is: {}", x);
}

/**
FUNCTION WITH RETURN VALUES
*/

fn five() -> u32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
