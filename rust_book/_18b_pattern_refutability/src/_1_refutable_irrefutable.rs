///# REFUTABLE VS IRREFUTABLE
/// - irrefutable pattern: ***let*** and ***for***
/// - refutable & irrefutable pattern: ***if let*** and ***while let***

#[test]
fn refutable_and_irrefutable() {
    let abc: Option<i32> = None;
    if let Some(x) = abc {
        println!("{}", x);
    }

    //if we give "if let" a pattern will always match, compiler give a warning
    if let x = 5 {
        println!("{}", x);
    }
}

#[test]
fn irrefutable() {
    // let abc: Option<i32> = None;
    // let Some(x) = abc;
}