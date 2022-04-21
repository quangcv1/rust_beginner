fn say(s: String) {
    println!("I say, {}", s);
}

fn main() {
    println!("Hello, world!");
    //Moving ownership with non-primitive
    let a = String::from("hello");
    say(a.clone());
    println!("a is {}", a);

    let s = String::from("book");
    //Add code here that calls the pluralize function
    println!(
        "I have one {}, you have two {}",
        s.clone(),
        //you add sth here,
    )
}
//add appropriate parameters, return values, and implementation to this function
fn pluralize(mut s: String) {
    s.push_str("s")
}
