///# RECURSIVE TYPE with Box
/// - At compile time, Rust needs to know how much space a type takes up
/// - One type whose size can't be known at compile time is ***recursive type***
/// - Rust doesn't know how much space a value of a recursive type needs
/// - However, boxes have a known size, so by inserting a box in a
/// recursive type definition, can have recursive types

///# Cons List
/// - Cons == ***construct function***
/// - constructs a new pair from its 2 arguments, which are a single value and
/// another pair
/// - "to cons x onto y" == construct a new container instance by
/// putting element x at the start of this new container, followed by
/// the container y.
/// - contains the value of the **the current item** and the **the next item**
/// - **last item** in the list contains only a value called ***Nil***
/// - is produced by recursively calling the ***cons*** function

///# Cons List with Box<T>
/// - ***Box<T>*** is a pointer, Rust knows how much space a Box<T> needs
/// - a pointer's size doesn't change based on the amount of data it's pointing to
/// - Put a ***Box<T>*** inside the ***Cons*** variant instead of another
/// ***List*** value directly => indirectly way
/// - ***Box<T> will point to the next ***List*** value that will be on the heap
/// rather than inside the ***Cons*** variant


#[test]
fn using_box_to_get_a_recursive_type_with_a_known_size() {
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    //recursive
    let list = List::Cons(1,
    Box::new(List::Cons(2,
    Box::new(List::Cons(3,
    Box::new(List::Nil))))));
}