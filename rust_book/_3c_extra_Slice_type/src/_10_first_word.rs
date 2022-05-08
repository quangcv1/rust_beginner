#[test]
fn first_word_before_slice() {
    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }

        s.len()
    }

    let mut s = String::from("hello world");

    let word = first_word(&s);
    //s.clear();
    println!("{}, {}", word, s);
    let word = &s[0..5];
    println!("{}", word);
}

#[test]
fn first_word_slice() {
    fn first_word(s: &String) -> &str {
        println!("{}", s);
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..i];
            }
        }

        println!("{}", s);
        &s[..]
    }

    let mut s = String::from("hello world");
    let word = first_word(&s); //immutable borrow here
    //s.clear(); clear needs to truncate the String, it needs
    //to get a mutable ref => error
    println!("the first word is: {}", word);
}

#[test]
fn string_slice_para_() {
    fn first_word_(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..i];
            }
        }

        &s[..]
    }

    //String
    let s = String::from("hello world");
    let word = first_word_(&s);

    println!("{}", word);
    //String_literal
    let string_literal = "hello world";
    let word = first_word_(string_literal);
    println!("{}", word);
    let word = first_word_(&string_literal);
    println!("{}", word);
}