fn main() {
    println!("Hello, OWNERSHIP!");
    /**
    STACK vs HEAP
    */

    /**
    OWNERSHIP:
    - Each value has a variable that's called its owner.
    - There can only be one owner at a time
    - When the owner goes out of scope, the value will be dropped. Rust calls **drop** automatically at the closing curly bracket.
    */

    /**
    STRING TYPE
    - data is stored on the heap and how Rust knows when to clean up that data
    - String value is hardcoded and immutable
    - For example: what if we want to take user input and store it ? => SECOND STRING TYPE **String**
    - **String** Type:
        - manages data allocated on the heap and is able to store an amount of text that is unknown at compile time.
    */
    let a = String::from("hello"); //**String::from** requests the memory it needs.
    let mut s = String::from("Hello Quang");
    s.push_str(", world!");
    println!("{}", s);

    let b = "b";
    //println!("{}", b);
    //b = "quang";
    println!("{}", b);
    /**
    Scope with **String**
    */
    let s1 = String::from("hello");
    let s2 = s1;
    //println!("{}, world!", s1); //value borrowed at s1 after move

    /**
    CLONE with **String**
    */
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    /**
    STACK-ONLY DATA: COPY
    - All integer types
    - All floating types
    - Character type
    - Tuples but not with **String** (ex: (i32, String))
   */
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
}
