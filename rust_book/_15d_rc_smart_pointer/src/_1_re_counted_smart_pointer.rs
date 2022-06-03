use std::rc::Rc;

///# Rc<T>
/// - A single value might have multiple owners
/// - Rc<T> type keeps track of the number of refs to a value to determine
///whether or not the value is still in use
///if there are zero refs to a val, the val can be cleaned up
///without any ref becoming invalid.

///# USE CASE:
/// - when we want to allocate some data on the heap for
///multiple parts of our program to read
///and we can't determine at compile time which part will finish using the data last
/// - if we knew which part would finish last, we could just make
/// that part the data's owner, and the normal ownership rules enforced at
/// compile time would take effect.

#[test]

///![image](https://doc.rust-lang.org/book/img/trpl15-03.svg)
fn using_rc_to_share_data_err() {
    // enum List {
    //     Cons(i32, Box<List>),
    //     Nil,
    // }
    //
    // let a = List::Cons(5, Box::new(List::Cons(10,
    // Box::new(List::Nil))));
    // let b = List::Cons(3, Box::new(a));
    // let c = List::Cons(4, Box::new(a));
}

#[test]
fn using_rc_to_share_data() {
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    //create the list holding 5 and 10 and store it in a
    //new Rc<List>
    let a = Rc::new(List::Cons(5,
    Rc::new(List::Cons(10,
    Rc::new(List::Nil)))));

    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = List::Cons(3,Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    //Rc::clone doesn't make a deep copy of all the data
    //but only increments the ref count, which doesn't take much time
    {
        let c = List::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

}