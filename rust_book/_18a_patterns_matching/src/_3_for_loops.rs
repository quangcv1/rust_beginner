#[test]
fn for_loops() {
    let v = vec!['a','b','c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    for val in v.iter() {
        println!("{}", val);
    }
}