///# Same function on both ***&String*** values and ***&str*** values
///- ***Implicit Defer Coercions with Functions and Methods

#[test]
fn string_slice_para() {
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..i];
            }
        }

        &s[..]
    }

    let s = String::from("hello world");
    let word = first_word(&s);

    println!("{}", word);
}