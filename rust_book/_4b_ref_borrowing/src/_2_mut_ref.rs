///# MUTABLE REFERENCES
/// - Only referring to mutable variables ***mut s*** and ***&mut s***
///
/// > Big restriction:
/// >Only one mutable ref to a particular piece of dat at a time
/// >Error when create >= ***2 mut ref*** to ***a ref var***
///
/// # DATA RACING
/// - 2 or more pointers access the same data at the same time
/// - At least one of the pointers is being used to write to the data
/// - There's no mechanism being used to sync access to the data
#[test]
fn mut_ref() {
    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }

    let mut s = String::from("hello");
    change(&mut s);
}


#[test]
fn _2_mut_ref_err() {
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // //Data racing
    // println!("{}", r1);
    // //println!("{}, {}", r1, r2);
}

#[test]
fn _2_mut_ref_err_tricky() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    }//r1 goes out of scope here
    let r2 = &mut s;
    println!("{}", r2);
}

#[test]
fn ref_mut_ref_err() {
    // let mut s = String::from("hello");
    // let r1 = &s; // no problem
    // let r2 = &s; //no problem.
    // let r3 = &mut s; //BIG PROBLEM
    //
    // //println!("{}", r2); //OK
    // println!("{}, {} and {}", r1, r2, r3);
}