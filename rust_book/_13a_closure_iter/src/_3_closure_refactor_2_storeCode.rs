use std::thread;
use std::time::Duration;

///# REFACTORING: With Closures to Store Code
/// Instead of always calling fun before the if blocks
/// Define a closure and store closure in a variable than storing the result
#[test]
///Still calling the expensive code twice
fn closure_refactor_store_code() {
    fn generate_workout(intensity: u32, random_number: u32) {
        let expensive_closure = |num| {
            println!("Calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num
        };

        if intensity < 25 {
            println!("Today, do {} pushups!", expensive_closure(intensity));
            println!("Next, do {} situps!", expensive_closure(intensity));
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!(
                    "Today, run for {} minutes!",
                    expensive_closure(intensity)
                )
            }
        }
    }

    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}