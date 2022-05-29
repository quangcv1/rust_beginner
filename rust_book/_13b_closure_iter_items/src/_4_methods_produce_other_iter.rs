#[test]
fn method_produce_other_iter() {
    let v1 = vec![1,2,3];
    //let v1_iter_new: Vec<_> = v1.iter().map(|x| x+1).collect();
    let v2: Vec<_> = v1.iter().map(|x| x+1).collect();
    //collect the resulting values into a collection data type
    println!("v2: {:?}", v2);
    println!("v1: {:?}", v1);
    for val in v2 {
        println!("{}", val);
    }
}