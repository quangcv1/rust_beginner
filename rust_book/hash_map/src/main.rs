use std::collections::HashMap;
fn main() {
    println!("Hello, HasMap!");
    //Creating a New Has map
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}",scores);
    //Creating a Hasmap from vector
    let teams = vec![String::from("Blue"),String::from("Yellow")]; //vector of tuple
    let initial_scores = vec![10,50]; //vector of tuple
    print!("{:?}",teams);
    let  scores_from_vec: HashMap<_,_> = teams.into_iter()
        .zip(initial_scores.into_iter())
        .collect();
    //println!("{:?}",teams);
    //println!("{:?}",initial_scores);
    println!("{:?}",scores_from_vec);

    //HASH MAPS AND OWNERSHIP
    let field_name = String::from("Quang");
    let age = 38;
    let mut map = HashMap::new();
    map.insert(field_name,age);
    println!("{:?}", age); //Copy trait for i32 data type
    //println!("{:?}", field_name);  //owner is moved to hash map

    //ACCESSING VALUES IN A HASH MAP
    let team_name = String::from("Blue");
    let score_team = scores.get(&team_name);
    println!("Blue score is {:?}",score_team);
    println!("{:?}", scores);

    //UPDATING A HAS MAP
    //1. Overwrite a value
    scores.insert(String::from("Blue"),5);
    println!("{:?}", scores);

    //2. Only insert a value if the key has no value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"),10);
    println!("{:?}", scores);
    scores.entry(String::from("Yellow"))
        //.or_insert(50);
        //.or_default();
        .or_insert_with(|| 20);
        //.or_insert_with_key("Red");

    scores.entry(String::from("Blue"))
        //.or_insert(50);
        .or_insert_with(||5);
    println!("{:?}", scores);

    //updating a value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}",map);

    let mut map: HashMap<&str, String> = HashMap::new();
    let s = "hoho".to_string();
    println!("{:?}",map);
    map.entry("poneyland").or_insert_with(|| s);
    println!("{:?}",map);

    assert_eq!(map["poneyland"], "hoho".to_string());
}
