use std::thread;
use std::time::Duration;

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

#[test]
/// Main thread will wait for the spawned thread to finish
/// and then run its for loop
fn waiting_for_all_threads_to_finish_using_join_2() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Spawn thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("Main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    };
}