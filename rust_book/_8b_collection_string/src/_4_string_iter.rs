#[test]
fn string_iter_chars() {
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
}

#[test]
fn string_iter_bytes() {
    for c in "नमस्ते".bytes() {
        println!("{}", c);
    }

    for c in "नमस्ते".as_bytes() {
        println!("{}", c);
    }
}