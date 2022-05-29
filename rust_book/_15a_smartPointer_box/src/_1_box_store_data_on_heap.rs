///# Box<T>: To Point to Data on the Heap
///- Type is written ***Box<T>***
///- store data on the heap rather than the stack
///- What remains on the stack is the pointer to the heap data - Refer Chap 04
///- Don't have performance overhead, other than storing data on the heap
///- is a pointer, Rust always knows how much space a ***Box<T>*** needs
///- a pointer's size doesn't change based on the amount of data it's pointing to

///# BOX: SCOPE
/// - When a box goes out of scope, it will be deallocated
/// - The deallocation happens for the box (stored on the stack)
/// and the data it points to (stored on the heap)

///# BOX TYPE: implement the ***traits***
/// - ***Deref*** trait: allows Box values to be treated like references
/// - => Box value goes out of scope, the heap data that is pointed to is cleaned
/// up as well because of the ***Drop** trait 

///# USE CASES
///- A type whose size can't be known at compile time, and want to use a value of
///that type in a context that requires an exact size
///- A large amount of data and want to transfer ownership but ensure
///the data won't be copied when do so
///- Want to own a value and care only that it's a type that implements
///a particular trait rather than being of a specific type

///# RECURSIVE TYPE with Box
/// - At compile time, Rust needs to know how much space a type takes up
/// - One type whose size can't be known at compile time is ***recursive type***
/// - Rust doesn't know how much space a value of a recursive type needs
/// - However, boxes have a known size, so by inserting a box in a
/// recursive type definition, can have recursive types

#[test]
fn box_store_data_on_the_heap() {
    //b have the value of a Box that points to the value 5
    //which is allocated on the heap
    let b = Box::new(5);
    println!("b = {}", b);
}