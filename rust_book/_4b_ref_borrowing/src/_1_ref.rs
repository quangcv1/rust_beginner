/// # REF
/// ***&s1*** create a ref that refers to the value of ***s1*** but doesn't own it.
/// *Because it does not own it, the value it points to will not dropped when the ref stops being used*
/// > We call the action of creating ref ***borrowing***
#[test]
fn refe() {
    fn calculate_length(s: &String) -> usize {
        s.len()
    } // Here, s goes out of scope. But it doesn't have ownership
    //of what it refers to, nothing happens.
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}
