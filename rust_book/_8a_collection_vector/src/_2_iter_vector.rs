#[test]
fn iter_values_in_a_vector() {
    //immutable
    let v = vec![100,32,57];
    for i in &v {
        println!("{}", i);
    }

    //mutable
    let mut v = vec![100,32,57];
    for i in &mut v {
        *i +=50; //dereference operator to get the value in ***i*** before we can use +=
        println!("{}", i);
    }

    println!("{:?}",v);
}