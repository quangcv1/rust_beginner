#[test]
fn let_statements() {
    let x = 5;
    let (x,y,z) = (1,2,3);
    //let (x,y) = (1,2,3) //error here
    let (x,y,_) = (1,2,3);
}