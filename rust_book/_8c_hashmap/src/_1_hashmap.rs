use std::collections::HashMap;

#[test]
fn hashmap_create() {
    let mut scores = HashMap::new();

    scores.insert("Blue".to_string(), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);
}

#[test]
fn hashmap_create_iter() {
    let teams = vec!["Blue".to_string(), "Yellow".to_string()];
    let initial_scores = vec![10,50];

    let mut scores: HashMap<_,_> = teams.into_iter()
        .zip(initial_scores.into_iter())
        .collect();
    println!("{:?}", scores);
}

#[test]
fn hashmap_ownership() {
    let field_name = "Fav".to_string();
    let field_value = "blue".to_string();

    let mut map = HashMap::new();
    map.insert(field_name,field_value);
    println!("{:?}", map);
    //println!("{}", field_name);//error
}

