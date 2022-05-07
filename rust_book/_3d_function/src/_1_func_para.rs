#[test]
fn func_para() {
    fn another_function(x: i32) {
        println!("The value of x is: {}", x);
    }
    another_function(5);
}

#[test]
fn func_para_2() {
    fn print_labeled_measurement(value: i32, unit_label: char) {
        println!("The measurement is: {}{}", value, unit_label);
    }
    print_labeled_measurement(5,'h');
}