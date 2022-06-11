use std::sync::mpsc;
use std::thread;

/// - mpsc: multiple producer, single consumer
///

#[test]
fn tx_rx_mpsc_channel() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // error here: once the value has been sent to another thread
        // that thread could modify or drop it before we try to use the value again
        // other thread's modification could cause errors or
        // unexpected results due to inconsistent or nonexistent data
        //println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}