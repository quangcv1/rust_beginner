#[test]
fn tuple_type() {
    let tup: (i32,f64,u8) = (500,6.4,1);

    let tup = (500,6.4,1); //shadowing

    let (x,y,z) = tup; //destructuring
    println!("The value of y is: {}", y);

}

#[test]
fn tuple_access_directly() {
    let x = (500,6.4,1);
    let five_hundred = x.0;
    let six_point_for = x.1;
    let one = x.2;
    println!("{} {} {}", five_hundred, six_point_for,one);
}