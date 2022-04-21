fn main() {
    println!("Hello, Control Flow!");
    /**
    USING IF IN A LET STATEMENT
    */
    let condition = true;
    let number = if condition {5} else { 6 };
    println!("The value of number is: {}", number);

    /**
    LOOP
    */
    let mut count = 0;
    'counting_up_test: loop {
        println!("count = {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up_test;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);

    /**
    RETURNING VALUES FROM LOOP
    */
    let mut counter = 0;
    let result = loop {
        counter +=1;
        if counter == 10 {
            break counter * 2; //note
        }
    };
    println!("The result is {}", result);

    /**
    WHILE
    */
    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    /**
    LOOP THROUGH A COLLECTION
   */
    let a = [10,20,30,40,50];
    for element in a {
        println!("the value is: {}", element);
    }

    /**
    REVERSE LOOP
   */
    for number in (1..4).rev() {
        println!("{}", number);
    }
    println!("LIFTOFF!!!");
    for number in 1..4 {
        println!("{}", number);
    }
    println!("LIFTOFF!!!");
}
