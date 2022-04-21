fn main() {
    println!("Hello, world!");
    /**
    # Reference & Borrowing
    ![This is an image](https://doc.rust-lang.org/book/img/trpl04-05.svg)
    */
    let s1 = String::from("Hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);

    /**
    # Mutable References
    */
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    /**
    # DATA RACE
    - Two or more pointers access the same data at the same time
    - At least one of the pointers is being used to write to the data
    - There's no mechanism being used to synchronize access to the data
    */
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    println!("{}", r1);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world")
}