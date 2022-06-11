#[test]
fn fn_para() {
    fn print_coordinates(&(x,y): &(i32,i32)) {
        println!("Current location: ({},{})", x,y);
    }

    let point = (3,5);
    print_coordinates(&point);
}