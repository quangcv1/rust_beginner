#[test]
fn running_code_on_cleanup_with_the_Drop_Trait() {
    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data`{}`!", self.data);
        }
    }

    let c = CustomSmartPointer {
        data: "my stuff".to_string(),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created.");
}