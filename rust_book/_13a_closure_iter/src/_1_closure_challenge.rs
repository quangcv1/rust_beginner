use std::thread;
use std::time::Duration;

///# CLOSURES: An anonymous Fns That Can Capture Their Environment
/// - Closures are anonymous fns
/// - Can save in a variable
/// - Can pass as arguments to other fns
/// - Can capture values from the scope in which they're defined

#[test]
///***simulated_expensive_calculation** called 3 times
/// Cause to slow program
fn closure_challenge() {
    fn simulated_expensive_calculation(intensity: u32) -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity
    }

    fn generate_workout(intensity: u32, random_number: u32) {
        if intensity < 25 {
            println!(
                "Today, do {} pushups!",
                simulated_expensive_calculation(intensity)
            );
            println!(
                "Next, do {} situps!",
                simulated_expensive_calculation(intensity)
            );
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!(
                    "Today, run for {} minutes!",
                    simulated_expensive_calculation(intensity)
                );
            }
        }
    }

    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

}