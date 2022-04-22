/**
# DEFINE STRUCT
*/
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

/**
# TUPLE STRUCTS WITHOUT NAMED FIELDS TO CREATE DIFFERENT TYPES
*/
struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

/**
# UNIT-LIKE STRUCTS WITHOUT ANY FIELDS
*/
struct AlwaysEqual;

fn main() {
    println!("Hello, world!");
    let mut user1 = User {
        active: true,
        username: String::from("someone@example.com"),
        email: String::from("someusername123"),
        sign_in_count: 1,
    };
    println!("{}", user1.email);
    user1.email = String::from("anotheremail@example.com");
    println!("{}", user1.email);

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    //println!("{:#?}", user1);
    println!("{}", user1.sign_in_count);
    println!("{:#?}", user2);

    /**
    # STRUCT UPDATE SYNTAX
    */
    let user3 = User {
        email: String::from("another2@example.com"),
        ..user2
    };

    /**
    # TUPLE STRUCT
    */
    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    /**
    # UNIT-LIKE STRUCT
    */
    let subject = AlwaysEqual;
}

/**
# FIELD INIT SHORTHAND
*/
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}