#[test]
fn iter_sum() {
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    let total:i32 = v1_iter.sum();
    //error here
    // for val in v1_iter {
    //     println!("{}", val);
    // }
    //because sum takes ownership of the iter we call it on
    assert_eq!(total, 6);
}