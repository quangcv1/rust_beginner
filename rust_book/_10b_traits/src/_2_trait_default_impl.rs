#[test]
fn trait_default_impl() {
    pub trait Summary {
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    //use a default implementation
    impl Summary for NewsArticle {}

    let article = NewsArticle {
        headline: "headline".to_string(),
        location: "location".to_string(),
        author: "author".to_string(),
        content: "content".to_string(),
    };
    println!("New article available! {}", article.summarize());
}