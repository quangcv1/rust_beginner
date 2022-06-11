#[test]
fn matching_literals() {
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("any"),
    }
}

#[test]
fn matching_named_var() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        //new y var, not the y declared at the beginning with the val 10
        Some(y) => println!("Matched, y = {}", y),//shadowed variable y
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x={:?}, y = {}", x, y);
}