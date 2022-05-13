#[test]
fn trait_define() {
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})",
                self.headline,
                self.author,
                self.location,
            )
        }
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    let tweet = Tweet {
        username: "horse_ebook".to_string(),
        content: "of course, as you probably already know, people".to_string(),
        reply: false,
        retweet: false
    };

    println!("1 new tweet: {}", tweet.summarize());
}