use std::sync::Mutex;

///# MUTEXES: Allow Access to Data from One Thread at a Time
/// - Mutex: mutual exclusion
/// - Allows only one thread to access some data at any given time
/// - Mutex is described as ***guarding*** the data it holds via the locking system

///# ACCESS DATA WITH MUTEX
/// - A thread must first signal that it wants access by asking to
/// acquire the mutex's ***lock***

///# LOCK ?
/// - lock is a data structure that is part of the mutex that keeps track
/// of who currently has exclusive access to the data.

///# MUTEX 2 RULES
/// - You must attempt to acquire the lock before using the data
/// - When you're done with the data that the mutex guards, you must unlock the
/// data so other threads can acquire the lock
/// - Thanks to Rust's type system and ownership rules, you can't get locking
/// and unlocking wrong

#[test]
/// - lock() returns a smart pointer called ***MutexGuard***
/// - The ***MutexGuard*** implements ***Defer*** to point at our inner data
/// - Also has a ***Drop*** impl that releases the lock automatically when
/// goes out of scope
fn mutex_in_single_threaded() {
    let m = Mutex::new(5);

    {
        //Mutex<T> is a smart pointer
        let mut num = m.lock().unwrap();
        //deref with MutexGuard and assign value
        *num = 6;
    } // the lock release automatically at here (goes out of scope)

    println!("m = {:?}", m);
}