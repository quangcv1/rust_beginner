#[derive(Debug)]
pub struct Post {
    content: String,
}

#[derive(Debug)]
pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

#[derive(Debug)]
pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

#[test]
fn test() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    println!("{:?}", post);
    let post = post.request_review();
    println!("{:?}", post);
    let post = post.approve();
    println!("{:?}", post);
    assert_eq!("I ate a salad for lunch today", post.content());
}