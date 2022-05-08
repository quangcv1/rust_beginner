#[test]
fn instance_from_other_instance() {
    #[derive(Debug)]
    struct User {
        active: bool,
        username: String,
        //why String not &String because we
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

    let user2 = User {
        active: user1.active,
        username: user1.username, //after this, we can no longer
        //user user1
        email: String::from("abc@abc.com"),
        sign_in_count: user1.sign_in_count
    };

    //println!("{:#?}", user1);//error here
    println!("{:#?}", user2);

    let user3 = User {
        email: String::from("abc"),
        ..user2
    };
    println!("{:#?}", user3);


}




