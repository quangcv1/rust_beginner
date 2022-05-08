#[test]
fn define_struct() {
    #[derive(Debug)]
    struct User {
        active: bool,
        username: String, //why String not &String because we
        //want it has owner of full String
        email: String,
        sign_in_count: u64,
    }
    //use a struct
    let mut user1 = User {
        active: true,
        username: String::from("a"),
        email: String::from("a@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("b@example.com");
    println!("{:#?}", user1);
}

#[test]
//with build user
fn define_struct_instance_with_func() {
    #[derive(Debug)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username, //Field init Shorthand
            email, //Field init Shorthand
            sign_in_count: 1,
        }
    }

    let user1 = build_user("a@a.com".to_string(), "ab".to_string());
    println!("{:#?}", user1)

}