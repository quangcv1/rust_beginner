#[test]
fn process_items_with_iter() {
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    println!("v1: {:?}", v1);
    for val in v1_iter {
        println!("Got: {}", val);
    }
    for val in v1 {
        println!("Got v1: {}", val);
    }
}