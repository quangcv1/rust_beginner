///# DANGLING REF
///- A pointer that references a location in mem that have been given to someone else

#[test]
fn test_dangling_ref() {
    // fn dangle() -> &String { //return a ref to a String
    //     let s = String::from("hello"); // s is a new String
    //     &s //return a ref to the String, s
    // } // here, s goes out of scope and is dropped.
    // //It's memory goes any => Danger
    //
    // let reference_to_nothing = dangle();
}

#[test]
fn dangling_ref_tricky() {
    fn no_dangle() -> String {
        let s = String::from("hello");
        s
    }

    let str = no_dangle();
    println!("{}", str);
}