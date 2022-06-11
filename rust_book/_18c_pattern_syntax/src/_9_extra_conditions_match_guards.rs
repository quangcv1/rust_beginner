#[test]
fn extra_conditionals_with_match_guards() {
    let num = Some(4);

    match num {
        Some(x) if x%2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }
}

#[test]
fn extra_conditionals_with_match_guards_2() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x,y);
}

#[test]
fn extra_conditionals_with_match_guards_3() {
    let x = 4;
    let y = true;

    match x {
        4 | 5| 6 if y => println!("Yes"),
        _ => println!("no"),
    }
}



