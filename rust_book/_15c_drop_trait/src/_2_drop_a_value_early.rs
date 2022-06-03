///# USE CASE: Want to clean up a value early
/// - Rust doesn't let you call the ***Drop*** trait's ***drop*** method manually
/// - Have to call the ***std::mem::drop*** function to force a value to be dropped
/// before the end of its scope.


#[test]
fn dropping_a_val_early() {
    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data`{}`!", self.data);
        }
    }

    let c = CustomSmartPointer {
        data: "some data".to_string(),
    };

    println!("CustomSmartPointers created.");
    std::mem::drop(c); // not c.drop()
    println!("CustomSmartPointer dropped before the end of main.");
}