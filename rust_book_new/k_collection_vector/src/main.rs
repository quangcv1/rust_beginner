fn main() {
    /**
    # COMMON COLLECTIONS
    - Unlike the built-in array and tuple types
    - the data these collections point to is stored on the heap, which means:
    - the amount of data does not need to known at compile time
    - can grow or shrink as the program runs
    ## COLLECTION TYPES:
    - A ***vector*** allows you to store a variable number of values next to each other
    - A ***string*** is a collection of characters.
    - A ***hash map*** allows you to associate a value with a particular key
   */

    /**
    # VECTOR
    - store more than one value in a single data structure
    - put all the values next to each other in memory
    - only store values of the same type
   */
    let v1: Vec<i32> = Vec::new();
    let v = vec![1,2,3]; //vec macro create a new vector that holds the values you give it
    /**
    ## UPDATING A VECTOR
   */
    let mut v2 = Vec::new();
    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);

    /**
    ## DROPPING A VECTOR
   */
    {
        let v = vec![1,2,3,4];
        //do stuff with v
    } // <- v goes out of scope and is freed here

    /**
    ## READING ELEMENTS
   */
    let v = vec![1,2,3,4,5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There's no third element"),
    }

    /**
    ## ITERATING OVER THE VALUES IN A VECTOR
   */
    let v = vec![100,32,57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100,32,57];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    /**
    ## USING AN ENUM TO STORE MULTIPLE TYPES
   */
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
