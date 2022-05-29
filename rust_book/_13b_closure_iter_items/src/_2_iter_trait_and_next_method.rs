#[test]
fn iter_trait_and_next_methods() {
    // pub trait Iterator {
    //     type Item; //associated type
    //     fn next(&mut self) -> Option<Self::Item>;
    //     //methods with default implementations elided
    // }

    let v1 = vec![1,2,3];
    let mut v1_iter = v1.iter();
    //calling next on an iter changes ***internal state*** that
    //iter uses to keep track of where it is in the sequence
    //no need to make iter mutable when use a ***for*** loop
    //because the loop take ownership of iter and made it mutable behind the scenes
    println!("v1: {:?}", v1);
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn into_iter_next() {
    let v1 = vec![1,2,3];
    //iter take ownershop of v1 and returns owned values
    let v1_into_iter = v1.into_iter();
    //println!("v1: {:?}", v1); //error here
    for val in v1_into_iter {
        println!("Got: {}", val);
    }

}

#[test]
fn iter_mut_test() {
    let mut v1 = vec![1,2,3];
    let v1_iter_mut = v1.iter_mut();
    for val in v1_iter_mut {
        *val = *val + 1;
    }

    println!("v1: {:?}", v1);

}