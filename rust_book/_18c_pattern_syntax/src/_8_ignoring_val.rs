#[test]
fn ignoring_entire_val() {
    fn foo(_:i32, y:i32) {
        println!("This code only uses the y para: {}", y);
    }
    foo(3,4);
}

#[test]
fn ignore_parts_of_a_val_with_a_nested() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        //don't need to match on or use the values inside
        //either Some variant
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        },
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);
}

#[test]
fn ignore_parts_of_a_val_with_a_nested_2() {
    let numbers = (2,4,8,16,32);
    match numbers {
        (first,_,third,_,fifth) => {
            println!("Some numbers: {}, {}, {}", first,third,fifth);
        }
    }
}

#[test]
fn ignore_unused_var_by_starting_its_name() {
    //basic case
    let _x = 5;
    let y = 10;

    //still match
    let s = Some(String::from("Hello"));
    if let Some(_s) = s { //moved here
        println!("found a string: {}", _s);
    }

    //println!("{:?}", s); //error here
}

#[test]
fn ignore_remaining_parts_of_a_val() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point {
        x: 0,
        y: 0,
        z: 0,
    };

    match origin {
        Point{x,..} => println!("x is {}", x),
    }

    //ignore between
    let numbers = (2,4,8,16,32);
    match numbers {
        (first,..,last) => {
            println!("Some numbers: {}, {}", first,last);
        }
    }
}