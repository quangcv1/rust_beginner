#[cfg(test)]
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

mod back_of_house {
    pub mod serving {
        pub fn cook() {}
    }
}
//
// use crate::front_of_house::hosting; //absolute path
// use self::front_of_house::hosting; //relative path
use front_of_house::hosting; //relative path
use back_of_house::serving as Serve; //new name with as keyword

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist(); //vs front_of_house::hosting::add_to_waitlist()
    hosting::add_to_waitlist();
    Serve::cook(); //Proving new names with as keyword
}