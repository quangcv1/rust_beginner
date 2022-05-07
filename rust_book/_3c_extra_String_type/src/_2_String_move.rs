#[test]
fn string_move_err() {
    let s1 = String::from("hello");
    let s2 = s1;

    //println!("{}, world", s1); //error here
    println!("{}, world", s2);
}

#[test]
fn heap_vs_stack_move() {
    //stack-only data: Copy
    //integers, bool, float, char, tuples (if only stack types)

    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x,y);

    //heap (String)
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}