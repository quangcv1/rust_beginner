#[test]
fn returning_types_that_implement_traits() {
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

    ///# RETURNING
    /// you can only use impl Trait if you’re returning a single type.
    /// For example, this code that returns either a NewsArticle or a Tweet with the return type specified as impl Summary wouldn’t work:
    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: "username".to_string(),
            content: "content".to_string(),
            reply: false,
            retweet: false,
        }
    }

    println!("returning...{}",returns_summarizable().summarize());
}