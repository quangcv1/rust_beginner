/// # SCALAR TYPES
/// ## Integer Types in Rust
/// | **Length** | **Signed** | **Unsigned** |
/// |:----------:|:----------:|:------------:|
/// | 8-bit      | i8         | u8           |
/// | 16-bit     | i16        | u16          |
/// | 32-bit     | i32        | u32          |
/// | 64-bit     | i64        | u64          |
/// | 128-bit    | i128       | u128         |
/// | arch       | isize      | usize        |
/// ## Integer Literals in Rust
/// | Number literals |   Example   |
/// |:---------------:|:-----------:|
/// | Decimal         | 98_222      |
/// | Hex             | 0xff        |
/// | Octal           | 0o77        |
/// | Binary          | 0b1111_0000 |
/// | Byte (u8 only)  | b'A'        |

#[test]
fn convert_type() {
    let guess: u32 = "42".parse().expect("Not a number");
    assert_eq!(42,guess);
    //let guess = "42".parse().expect("Not a number"); //error case
}
#[test]
fn float_type() {
    let x = 2.0; //f64 cause of OS 64 bits
    let y: f32 = 3.0; // f32
    assert_eq!(x,2.0);
    assert_eq!(y,3.0);
}
#[test]
fn numeric_ops() {
    //remainder
    let remainder = 43%5;
    assert_eq!(3,remainder);
}

#[test]
fn character_type() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{} \n {} \n {}", c,z,heart_eyed_cat);
}