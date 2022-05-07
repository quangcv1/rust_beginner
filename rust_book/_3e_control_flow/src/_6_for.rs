#[test]
fn for_collection() {
    let a = [10,20,30,40,50];
    for element in a {
        println!("the value is: {}", element);
    }
}

#[test]
fn for_normal() {
    for number in 1..4 {
        println!("{}", number);
    }
    println!("LIFTOFF");
}

#[test]
fn for_reverse() {
    for number in (1..4).rev() {
        println!("{}", number);
    }
    println!("LIFTOFF!!!");
}