#[test]
fn match_exhaustive() {
    // fn plus_one(x: Option<i32>) -> Option<i32> {
    //     match x { //match exhaustive here, because we didn't cover every possible case.
    //         Some(i) => Some(i + 1),;
    //
    //     }
    // }
}

#[test]
fn match_exhaustive_fix_with_placeholder() {
    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => re_roll(),
    }

    fn add_fancy_hat() {};
    fn remove_fancy_hat() {};
    fn re_roll() {};
}

#[test]
fn match_exhaustive_fix_with_unit_value() {
    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {};
    fn remove_fancy_hat() {};
}