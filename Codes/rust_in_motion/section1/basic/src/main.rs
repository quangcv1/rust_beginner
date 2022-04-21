use std::io;

fn main() {
    let x = 1;
    let y = 6;
    let mut z: i8 = x + y;
    z += 1;
    println!("z is {}", z);

    let tuple = (1,'c',true, "hello world");
    let first = tuple.0;
    let second = tuple.1;
    let (a,b,c,d) = tuple;
    println!("the first is {}", first);
    println!("the second is {}", second);
    println!("the third is {}", c);
    println!("the fourth is {}", d);

    next_birthday("Quang", 37);
    println!("Square of 3 is {}", square(3));
    discount(14);
    // input();
    // forLoop();
    matchTest(1);
    diceMatch(1,5);
    diceIgnore(2)

}

fn diceIgnore(num: i32) {
    match num {
        1 => println!("One"),
        _ => {},
    }
}

fn diceMatch(die1: i32, die2: i32) {
    match (die1, die2) {
        (1,1) => println!("1, 1"),
        (5,_) | (_,5) => {
            println!("You rolled at least one 5!");
            println!("Move and then roll again!");
        },
        _ => println!("Move your piece!"),
    }
}

fn input() {
    loop {
        println!("What's the secret word?");
        let mut word = String::new();
        io::stdin().read_line(&mut word).expect("Failed to read line");

        if word.trim()=="rust" {
            println!("Done Input!");
            break;
        }
    }
    let mut word = String::new();
    while word.trim()!="rust" {
        println!("What is the secret word?");
        word = String::new();
        io::stdin().read_line(&mut word).expect("Failed to read line");
    }
    println!("You know the secret world! Pls proceed!");
}
fn matchTest(num: u8) {
    match num {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Some other number"),
    }
}
fn forLoop() {
    for i in 1..11 {
        println!("Now serving number {}",i)
    }
}

fn next_birthday(name: &str, current_age: u8) {
    let next_age = current_age + 1;
    println!("Hi {}, on your next birthday, you'll be {}!", name, next_age);
}

fn square(num: i32) -> i32 {
    return num*num;
}

fn discount(day_of_month: u8) {
    let discount = if day_of_month % 2 == 0 {
        50
    } else {
        10
    };
    println!("Your discount is {}!", discount);
}