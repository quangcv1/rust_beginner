use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

///# REF CYCLES CAN LEAK MEM
/// - accidentally create memory that is never cleaned up (memory leak)
/// - Rust allows mem leaks by using Rc<T> and RefCell<T>
/// - it's possible to create ref where items refer to each other in a cycle
/// - This creates mem leaks because the ref count of each item in the cycle
/// will never reach 0, and the values will never be dropped.

#[test]
fn creating_a_ref_cycle() {
    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }

    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                List::Cons(_, item) => Some(item),
                List::Nil => None,
            }
        }
    }

    ///# Create ref cycles
    let a = Rc::new(
        List::Cons(5, RefCell::new(Rc::new(List::Nil)))
    );

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(List::Cons(10,
        RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    //change a points to b instead of Nil by
    // using tail(): to get ref of a and put in the var link
    // use borrow_mut() on RefCell to change the value inside from Nil to value in b
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    //println!("a next item = {:?}", a.tail());

}