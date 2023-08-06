// ? //////////////////////////////////////////////// 
// ? Structs go on the outside of the main
// ? //////////////////////////////////////////////// 
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Colour(String, String, String);
struct Point(i32, i32, i32);

fn main() {

    let mut user1 = User {
        username: String::from("Korkyjohnson"),
        email: String::from("korkyjohnson@gmail.com"),
        sign_in_count: 1,
        active: true,
    };

    user1.email = String::from("rdhanna@gmail.com");

    println!("user1.email: {}", user1.email);

    let user2 = build_user(String::from("bobandjannet@gmail.com"), String::from("JHanna88"));
    println!("user2.email: {}", user2.email);
    println!("user2: {:#?}", user2);
    
    let user3 = User{
        username: String::from("AnimeFlames"),
        email: String::from("gordon.hanna@gmail.com"),
        ..user1 // struct update syntax that uses the other properties of user1 to fill in the rest of the struct for user3
    };

    println!("user3.email: {}", user3.email);

    let colour1 = Colour(String::from("Red"),String::from("Green"),String::from("Blue"));
    let point1 = Point(9,4,7);

    println!("Colour.1: {}", colour1.1);
    println!("Point.0: {}", point1.0);

}

fn build_user(email: String, username: String) -> User {
    User {
        username, // don't have to go username: username because they have the same name
        email,
        active: true,
        sign_in_count: 1,
    }
}
