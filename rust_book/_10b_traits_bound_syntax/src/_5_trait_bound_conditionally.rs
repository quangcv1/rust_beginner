use std::fmt::Display;

#[test]
fn trait_bounds_to_conditionally_implement_methods() {
    struct Pair<T> {
        x: T,
        y: T,
    }

    impl <T> Pair<T> {
        fn new(x:T, y:T) -> Self {
            Self{x,y}
        }
    }
    impl <T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x > self.y {
                println!("The largest number is x = {}", self.x);
            } else {
                println!("The largest number is y = {}", self.y);
            }
        }
    }

    let pair = Pair::new(30,65);
    pair.cmp_display();

}