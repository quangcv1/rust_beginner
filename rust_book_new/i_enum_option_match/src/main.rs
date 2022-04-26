fn main() {
    /**
    # NONE COVER
    */
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    /**
    # CATCH-ALL PATTERNS AND THE _PLACEHOLDER
    */
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {};
    fn remove_fancy_hat() {};
    fn move_player(num_spaces: u8) {};


}

