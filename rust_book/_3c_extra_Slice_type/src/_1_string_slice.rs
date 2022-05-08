#[test]
fn string_slice() {
    let s = String::from("Hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{} {}", hello, world);
}

#[test]
fn slice_range_syntax() {
    //case 1
    let s = String::from("hello");

    let slice = &s[0..2];
    let slice1 = &s[..2];

    assert_eq!(slice, slice1);

    //case 2
    let len = s.len();

    let slice = &s[3..len];
    let slice1 = &s[3..];

    assert_eq!(slice, slice1);

    //case 3

}