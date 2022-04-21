use std::io;

fn main() {
    println!("Data Types!");
    /**
    SCALAR TYPES
    - int:
    - u8, u16, u32, u64, u128, usize
    - i8, i16, i32, i64, i128, isize
    - Decimal 98_222
    - Hex     0xff
    - Octal   0o77
    - Bi      0b1111_0000
    - Byte(u8 only) b'A'
    */
    let i = 32;

    /**
    FLOAT TYPES
    - f32: is a single-precision float
    - f64: double precision
    */
    let x = 2.0;
    let y:f32 = 3.0;
    /**
    BOOLEAN
    */
    let t = true;
    let f:bool = false; //explicit type annotation
    /** CHARACTER TYPE

    */
    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';

    /**
    COMPOUND TYPES
    - Tuple: grouping anumber of values with a variety of types into one
    - Unit type with Tuple: without any values, (), is a special type that has only one value, also written ().
    */
    let tup = (500, 6.4,1);
    let tup_ex: (i32,f64,u8) = (500,6.4,1);
    //Destructuring
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    //Accessing
    let x: (i32, f64, u8) = (500, 6.4,1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    /**
    ARRAY TYPE
    - every element of an array must have the same type
    - arrays in Rust have a fixed length
    */
    let a = [1,2,3,4,5];
    let a: [i32; 5] = [1,2,3,4,5]; //explicit
    let a = [3;5]; // === [3,3,3,3,3]
    //Accessing Array Elements
    let first = a[0];
    let second = a[1];

    /**
    INVALID ARRAY ELEMENT ACCESS
    */
    let a = [1,2,3,4,5];
    println!("Pls enter an array index.");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!(
        "The value of the element at index {} is: {}",
        index, element
    )
}
