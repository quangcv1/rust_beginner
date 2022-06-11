use std::thread;

#[test]
/// - move keyword before the closure
/// - to force the closure take ownership of the values it's using
/// rather than allowing Rust to infer that it should borrow the values
fn using_move_closures_with_threads() {
    let v = vec![1,2,3];
    let handle = thread::spawn(move || {
        println!("Spawn thread: {:?}", v);
    });

    handle.join().unwrap();
}
#[test]
/// - println! only needs a ref to v
/// - the closure tries to borrow v
/// - Problem: rust can't tell how long the spawned thread will run,
/// so it doesn't know if the ref to v will always be valid
fn testing_ownership_spawn() {
    // let v = vec![1,2,3];
    //
    // let handle = thread::spawn(|| {
    //     println!("Spawn thread: {:?}", v);
    // });
    //
    // handle.join().unwrap();
}

#[test]
/// - spawn thread would be immediately put in the background wo running at all
/// - Problem: spawned thread has a ref to v inside
/// - but the main thread immediately drops v
/// - when the spawned thread starts to execute, v is no longer valid
/// - so a ref to it is also invalid
fn testing_ownership_spawn_drop() {
    // let v = vec![1,2,3];
    //
    // let handle = thread::spawn(|| {
    //     println!("Spawn thread: {:?}", v);
    // });
    //
    // drop(v);
    //
    // handle.join().unwrap();
}