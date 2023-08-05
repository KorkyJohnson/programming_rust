fn main() {
    // let mut s = String::from("hello");

    // s.push_str(", world"); // push_str() appends a literal to a String

    // println!("{}", s); 

    // Can assign Integers via direct move, howver Strings cannot, must use .clone
    // let x = 5;

    // let _y = x;

    // println!("x: {}, y: {}", x, _y);
    
    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    
    // println!("s1: {}, s2: {}", s1,s2);

    // ? **************************************
    // ? Ownership and funcntions:
    // ? **************************************
    let s = String::from("hello");
    takes_ownership(s);

    let x = 5;

    makes_copy(x);
    
    let s1 = gives_ownership();
    
    let s2 = String::from("hello");
    
    let s3 = takes_and_gives_back(s2);
    
    println!("s1: {}", s1);
    // println!("s2: {}", s2); // won't print becuase we gave it away in takes_and_gives_back
    println!("s3: {}", s3);
    
    // ? **************************************
    // ? References and borrowing
    // ? **************************************


}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {

    let some_string = String::from("hello");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {

    a_string
}