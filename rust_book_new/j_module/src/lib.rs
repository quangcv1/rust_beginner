/**
# FRONT OF HOUSE:
- where customers are; this is where hosts seat customers, servers take orders and payment, and bartenders make drinks
# BACK OF HOUSE:
- where the chefs and cooks work in the kitchen, dishwashers clean up, and managers do administrative work.
*/
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

/**
# PATHS FOR REFERRING TO AN ITEM IN THE MODULE TREE
- An ***absolute path*** starts from a crate root by using a crate or a literal crate
- A ***relative path*** starts from the current module and uses self, super, or an identifier in the current module
*/

pub fn eat_at_restaurant_1() {
    //Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    //Relative path
    front_of_house::hosting::add_to_waitlist();
}


fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); //use super to go to the parent module of back_of_house, which in this case is crate, the root
    }
    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    pub enum  Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    //Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    //Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}