///# CREATING NEW THREAD WITH SPAWN
/// - thread::spawn fun and pass it a closure
/// - closure containing the code we want to run in the new thread
use std::thread;
use std::time::Duration;

#[test]
/// main thread ends cause to new thread end whatever new thread end or not
fn creating_new_thread_with_spawn() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("Spawn thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}

#[test]
/// main thread waits because of the call to handle.join()
/// and does not end until the spawned thread is finished.
fn waiting_for_all_threads_to_finish_using_join() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Spawn thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("Main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    };

    handle.join().unwrap();
}