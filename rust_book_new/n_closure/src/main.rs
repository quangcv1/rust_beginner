use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn main() {
    // let simulated_user_specified_value = 10;
    // let simulated_random_number = 7;

    let simulated_user_specified_value = 25;
    let simulated_random_number = 3;
    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            //simulated_expensive_calculation(intensity)
            expensive_result
        );
        println!(
            "Next, do {} situps!",
            //simulated_expensive_calculation(intensity)
            expensive_result
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes",
                //simulated_expensive_calculation(intensity)
                expensive_result
            );
        }
    }
}

