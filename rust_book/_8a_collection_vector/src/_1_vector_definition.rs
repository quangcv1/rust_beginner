///# VECTOR
/// - allow to store more than one value in a single dat structure
/// - puts all the values next to each other in memory.
/// - vectors can only store values of the same type
/// # THE WAY VECTOR WORKS
/// - vectors put the values next to each other in memory
/// - adding a new element onto the end of the vector might require allocating new mem
/// and copy the old elements to the new space, if there isn't enough room to put all
/// the elements next to each other where the vector is currently stored.
/// => the reference to the first element would be pointing to deallocated mem.
/// => the borrowing rules prevent programs from ending up in that situation

#[test]
fn create_vector() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1,2,3];

    println!("{:?}", v);
}

#[test]
fn update_vector() {
    let mut v: Vec<i32> = vec![1,2,3];
    v.push(4);
    println!("{:?}", v);
}

///# DROPPING A VECTOR DROPS ITS ELEMENTS
/// - a vector is freed when it goes out of scope
#[test]
fn dropping_vector() {
    {
        let v = vec![1,2,3,4];
    }
    //assert_ne!(v, vec![1,2,3,4]);
}

#[test]
fn reading_element() {
    let v = vec![1,2,3,4,5];

    //2 ways to references an element
    let first = &v[1];
    let first = v.get(1);

    let third = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }
}

#[test]
fn get_vs_square_bracket() {
    let v = vec![1,2,3,4,5];

    //let does_not_exist = &v[100]; // panic "index out of bounds..."
    let does_not_exist = v.get(100);
    println!("{:?}", does_not_exist); //not panic and return None
}

#[test]
fn deallocated_mem_case() {
    let mut v = vec![1,2,3,4,5];
    let first = &v[0];
    v.push(6);
    println!("{:?}", v);
    //println!("{:?}", first); //error
    // the way vectors work:

}