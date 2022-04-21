use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");
    let condition = true;
    let number = if condition {5} else {3};
    println!("{}",number);
    let secret_number = rand::thread_rng().gen_range(0..=100);
    println!("Your secret number is {}",secret_number);
    loop {
        println!("Pls input your guess number:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to readline");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Your guess number is: {}",guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Small"),
            Ordering::Greater => println!("Greater"),
            Ordering::Equal => {
                println!("Win");
                break;
            }
        }
    }


}
