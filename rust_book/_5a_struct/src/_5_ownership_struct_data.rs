#[test]
fn ownership_struct_data_error() {
    // #[derive(Debug)]
    // struct User {
    //     active: bool,
    //     username: &str,
    //     email: &str,
    //     sign_in_count: u64,
    // }
    //
    // let user1 = User {
    //     active: true,
    //     username: "a",
    //     email: "a@a.com",
    //     sign_in_count: 1,
    // };
    //
    // println!("{:#?}", user1)
}

#[test]
fn ownership_struct_data_fix() {
    #[derive(Debug)]
    struct User<'a> {
        active: bool,
        username: &'a str,
        email: &'a str,
        sign_in_count: u64,
    }

    let user1 = User {
        active: true,
        username: "a",
        email: "a@a.com",
        sign_in_count: 1,
    };

    println!("{:#?}", user1)
}