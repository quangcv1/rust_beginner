fn main() {
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }


    for val in vec![4,5,6].iter() {
        println!("Got: {}", val);
    }
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1,2,3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    for val in v1_iter {
        println!("Got: {}", val);
    }
}