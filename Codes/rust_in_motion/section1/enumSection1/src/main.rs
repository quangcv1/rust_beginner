use std::cmp::min;

fn main() {
    println!("Hello, world!");
    tell_time(Clock::Analog(14,12,52));
}

//enum's variants
enum HockeyPosition {
    Center,
    Wing,
    Defense,
    Goalie,
}

enum Clock {
    Sundial(u8),
    Digital(u8,u8),
    Analog(u8,u8,u8),
}

fn tell_time(clock: Clock) {
    match clock {
        Clock::Sundial(hours) => println!("It's about {} o'clock", hours),
        Clock::Analog(hours,minutes,seconds) => {
            println!(
                "It's {} minutes and {} seconds past {} o'clock",
                minutes, seconds, hours,
            );
        },
        Clock::Digital(hours,minutes) => println!("It's {} minutes past {}", minutes,hours,),
    };
}