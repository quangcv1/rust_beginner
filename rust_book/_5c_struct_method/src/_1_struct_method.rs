#[test]
fn struct_method() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    //method
    impl Rectangle {

        //constructor
        pub fn new(width: u32, height: u32) -> Self {
            Self { width, height }
        }

        //getter
        pub fn width(&self) -> u32 {
            self.width
        }
        pub fn height(&self) -> u32 {
            self.height
        }

        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    //Multiple impl blocks
    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    //do it
    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rec1.area()
    );

    //can hold
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rec1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rec1.can_hold(&rect3));

}

