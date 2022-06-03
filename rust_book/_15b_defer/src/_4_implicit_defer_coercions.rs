///#DEFER COERCIONS:
/// - Performs on arguments to ***functions and methods***
/// - Work only on types that implement the ***Defer*** trait
/// - Convert a reference to a type ***INTO*** another type
/// Ex: convert ***&String*** to ***&str*** because
/// ***String*** implements the ***Deref** trait such that it returns ***&str***
/// - Don't need to add as many explicit references and deref with & and *
/// - lets us write more code that can work for either references or smart pointers
use std::ops::Deref;

#[test]
fn implicit_deref_coercions() {
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

    fn hello(name: &str) {
        println!("Hello, {}", name);
    }

    let m = MyBox::new(String::from("Rust"));
    //&MyBox<String> into &String by calling deref
    hello(&m); //stand lib turns the &String into &str
}