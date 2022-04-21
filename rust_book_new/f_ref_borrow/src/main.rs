fn main() {
    println!("Hello, world!");
    /**
    ![image](/images/trpl04-05.svg)
    */
    let s1 = String::from("Hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}