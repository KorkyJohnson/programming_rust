// ? //////////////////////////////////////////////// 
// ? Structs go on the outside of the main
// ? //////////////////////////////////////////////// 
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Colour

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
    
    let user3 = User{
        username: String::from("AnimeFlames"),
        email: String::from("gordon.hanna@gmail.com"),
        ..user1 // struct update syntax that uses the other properties of user1 to fill in the rest of the struct for user3
    };

    println!("user3.email: {}", user3.email)
}

fn build_user(email: String, username: String) -> User {
    User {
        username, // don't have to go username: username because they have the same name
        email,
        active: true,
        sign_in_count: 1,
    }
}
