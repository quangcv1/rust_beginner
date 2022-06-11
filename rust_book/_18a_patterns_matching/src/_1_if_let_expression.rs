
#[test]
fn if_let_expression() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the bg", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age_ok) = age {
        if age_ok > 30 {
            println!("Using purple as the bg color");
        } else {
            println!("Using orange as the bg color");
        }
    } else {
        println!("Using blue as the bg color");
    }
}