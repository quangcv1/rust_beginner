///# METHODS ON A STRUCT WITH LIFETIMES
/// - on struct filed: need to be declared after ***impl***
/// - on method para: inside ***impl*** block
/// - on return value
///
#[test]
fn lifetime_annotations_in_struct() {
    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    impl <'a> ImportantExcerpt<'a> {
        //first rule
        fn level(&self) -> i32 {
            3
        }
    }

    impl <'a> ImportantExcerpt<'a> {
        //1st rule for self and announcement
        //3rd rule for return type get the lifetime of &self
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }

    let novel = "Call me. Some years ago...".to_string();
    let first_sentence = novel
        .split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("{:?} : {}", i, novel);
}