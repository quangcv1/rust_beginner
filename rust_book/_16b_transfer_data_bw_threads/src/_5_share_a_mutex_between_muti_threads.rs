use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
///# Atomic Reference Counting with Arc<T>
/// - Arc<T> is a type like Rc<T> that is safe to use in concurrent situations
/// - `a` stands for atomic

///# WHY IMPLEMENT TO USE Arc<T> by default
/// - The reason is that thread safety comes with a performance penalty that
/// you only want to pay when you really need to.
/// - if you're performing operations on values within single thread,
/// your code can run faster if it doesn't have to enforce the guarantees atomics provide

///# SIMILARITIES BW RefCell<T>/ Rc<T> and Mutex<T>/ Arc<T>
/// - counter is immutable and Mutex<T> provides interior mutability === RefCell
/// - Rust can't protect you from all kinds of logic errors when you use Mutex
/// - Rc<T> came with the risk of creating ***ref cycles*** -> mem leaks
/// - Mutex<T> comes with the risk of creating ***deadlocks***
#[test]
fn share_a_mutex_between_multi_threads() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num +=1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

#[test]
/// - can't move the ownership of lock counter into multi threads
/// - error: multiple-ownership
/// - Fix: with Rc<T>
fn share_a_mutex_bw_multi_threads_err_1() {
    // let counter = Mutex::new(0);
    // let mut handles = vec![];
    //
    // for _ in 0..10 {
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //
    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }
    //
    // for handle in handles {
    //     handle.join().unwrap();
    // }
    //
    // println!("Result: {}", *counter.lock().unwrap());
}

#[test]
/// - Rc<T> is not safe to share across threads
/// - it adds to the count for each call to clone and subtract from the count
/// when each clone is dropped. But it doesn't use any concurrency primitives
/// to make sure that changes to count can't be interrupted by another thread.
/// - Fix: need a type exactly like Rc<T> but one that makes changes to the ref count
/// in a thread-safe way.
fn share_a_mutex_bw_multi_threads_err_2() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}