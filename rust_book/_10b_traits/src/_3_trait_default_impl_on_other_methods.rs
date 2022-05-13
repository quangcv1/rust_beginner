#[test]
fn trait_default_impl_on_other_method() {
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

    let tweet = Tweet {
        username: "username".to_string(),
        content: "content".to_string(),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}