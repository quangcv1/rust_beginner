///# OWN SMART POINTER
///- Build a smart pointer similar to the ***Box<T>***

use std::ops::Deref;

#[test]
fn own_smart_pointer() {
    struct MyBox<T>(T); //tuple struct with one element of type T

    //takes one para of type T and returns a Mybox instance that holds the value passed
    //in
    impl <T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl <T> Deref for MyBox<T> {
        type Target = T;//defines an associated type for the Defer trait to use.
        //Associated types vs way of declaring a generic parameter

        fn deref(&self) -> &Self::Target {
            &self.0 //access the first value in a tuple struct
        }
    }

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5,*y); // === *(y.defer())
}