use std::collections::HashMap;

#[test]
fn hashmap_access() {
    let mut scores = HashMap::new();
    scores.insert("Blue".to_string(), 10);
    scores.insert("Yellow".to_string(), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&"Blue".to_string());
    println!("{:?}", score);
    let score = scores.get(&team_name);
    println!("{:?}", &score);
    println!("{:?}", &team_name);
}

#[test]
fn hashmap_access_for() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}