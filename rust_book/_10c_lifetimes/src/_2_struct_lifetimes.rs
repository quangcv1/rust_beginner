#[test]
fn lifetime_annotations_in_struct() {
    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str,
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