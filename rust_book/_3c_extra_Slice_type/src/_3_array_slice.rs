///# OTHER SLICES
///consider array

#[test]
fn array_slice() {
    let a = [1,2,3,4,5];
    let slice = &a[1..3];
    println!("{:?}", slice);
}