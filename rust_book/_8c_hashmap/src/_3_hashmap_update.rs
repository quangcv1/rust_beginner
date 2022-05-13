use std::collections::HashMap;

#[test]
fn insert_value_only_key_has_no_value() {
    let mut scores = HashMap::new();
    scores.insert("Blue".to_string(), 10);

    scores.entry("Yellow".to_string())
        .or_insert(50);
    scores.entry("Blue".to_string())
        .or_insert(50); //only insert this

    println!("{:?}",scores);
}

#[test]
fn update_value_on_old_value() {
    //normal
    let mut scores = HashMap::new();
    scores.insert("Blue".to_string(), 10);
    let value = scores.entry("Blue".to_string())
        .or_insert(0);
    *value = 100;
    println!("{:?}", scores);

    //complex with for
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}