#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    /**
    # METHOD
    */
    fn area(&self) -> u32 {
        self.width * self.height
    }

    /**
    # METHOD SAME NAME AS ONE OF THE STRUCT'S FIELDS
    */
    fn width(&self) -> bool {
        self.width > 0
    }

    /**
    # METHOD WITH MORE PARAS
    */
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    /**
    # ASSOCIATED FUNCTIONS
    */
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    println!("Hello, world!");
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let square1 = Rectangle::square(40);
    println!("{:#?}", square1);

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.can_hold(&rect2) {
        println!("The rect1 can hold rect2");
    }
}
