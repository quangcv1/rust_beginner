use std::thread;
use std::time::Duration;

///# CLOSURES: Store using Generic Para and the Fn Traits
///- Create struct hold the closure and the result value of calling closure
///- Struct execute the closure only if we need the resulting value
///- It will cache the resulting value
///- So the rest of code doesn't have to be responsible for saving and reusing the result
/// => This pattern called ***memorization*** or ***lazy evaluation***

///# CLOSURES: Fn traits bound to represent the types of the para and return values the closures must have
///All closures implement at least one of the traits:
///- Fn:
///- FnMut:
///- FnOnce: default

#[test]
fn closure_struct() {
    struct Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        calculation: T,
        value: Option<u32>,
    }
    //Before execute the closure, value will be None
    //When code using a Cacher asks for the result of the closure
    //The Cacher will execute the closure and store the result in Some
    //if the code asks for the result of the closure again,
    //The Cacher will return the result held in the Some
    impl <T> Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }
}

#[test]
fn closure_refactor_3_using_trait_bound() {
    struct Cacher<T>
    where
        T: Fn(u32) -> u32
    {
        calculation: T,
        value: Option<u32>,
    }

    impl <T> Cacher<T>
    where
        T: Fn(u32) -> u32
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }

    fn generate_workout(intensity: u32, random_number: u32) {
        let mut expensive_result = Cacher::new(
            |num| {
                println!("calculating slowly...");
                thread::sleep(Duration::from_secs(2));
                num
            }
        );

        if intensity < 25 {
            println!("Today, do {} pushups!", expensive_result.value(intensity));
            println!("Next, do {} situps!", expensive_result.value(intensity));
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!(
                    "Today, run for {} minutes",
                    expensive_result.value(intensity)
                )
            }
        }
    }

    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}