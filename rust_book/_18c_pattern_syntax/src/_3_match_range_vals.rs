#[test]
fn matching_ranges_of_vals() {
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';
    match x {
        'a' ..='j' => println!("early ASCII letter"),
        'k' ..='z' => println!("late ASCII letter"),
        _ => println!("Something else"),
    }
}