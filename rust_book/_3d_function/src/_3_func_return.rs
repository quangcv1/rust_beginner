#[test]
fn func_return() {
    fn five() -> i32 {
        5
    }

    let x = five();
    println!("The value of x is: {}", x);

    fn plus_one(x: i32) -> i32 {
        x + 1
    }

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}