#[test]
fn string_create() {
    let mut s = String::new(); //empty string
    s.push_str("abc");
    assert_eq!(s, "abc".to_string());

    //with initial data
    let data = "initial contents";
    let s = data.to_string(); //first way
    let s = "initial contents".to_string(); //the method also works on a literal directly

    //with String from
    let s = String::from("initial contents");
}

#[test]
fn update_string() {
    let mut s = "foo".to_string();
    s.push_str("bar");
    println!("{}", s);

    let mut s1 = "foo".to_string();
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {}, s2 is {}", s1, s2);//push method doesn't take ownership of s2
}