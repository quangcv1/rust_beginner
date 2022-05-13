use std::fmt::Display;

#[test]
fn trait_multi_trait_bounds() {
    pub trait Summary {
        fn summarize_author(&self) -> String;

        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    //we've implemented summarize_author, the Summary trait has given us
    //the behavior of the summarize method w/o requiring us to write any more code
    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("{}", self.username)
        }
    }

    ///# PARAMETERS
    ///Check it out
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }
    pub fn notify_2(item1: &impl Summary, item2: &impl Summary) {}
    pub fn notify_multi(item: &(impl Summary + Display)) {}

    ///# TRAIT BOUND SYNTAX
    /// this syntax convenient and makes for more concise code in simple cases.
    ///Check it out
    pub fn notify_bound<T:Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }
    pub fn notify_bound_2<T:Summary>(item1: &T, item2: &T) {    }

    ///# MULTIPLE TRAIT BOUNDS WITH + SYNTAX
    /// Check it out
    pub fn notify_multi_bound<T:Summary + Display>(item: &T) {}


    let tweet = Tweet {
        username: "username".to_string(),
        content: "content".to_string(),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    notify(&tweet);
}