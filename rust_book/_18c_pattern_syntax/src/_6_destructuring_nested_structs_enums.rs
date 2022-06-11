#[test]
fn destructuring_nested_structs_enums() {
    enum Color {
        Rgb(i32,i32,i32),
        Hsv(i32,i32,i32),
    }

    enum Message {
        Quit,
        Move {x: i32, y: i32},
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColor(Color::Hsv(0,160,255));

    match msg {
        Message::ChangeColor(Color::Rgb(r,g,b)) => println!(
            "Change the color to red {}, green {}, blue {}",
            r,g,b
        ),
        Message::ChangeColor(Color::Hsv(h,s,v)) => println!(
            "Change the color to hue {}, saturation {}, value {}",
            h,s,v
        ),
        _ => (),
    }
}