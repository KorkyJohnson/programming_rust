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

    let s4 = String::from("hello s4");

    let len = calculate_length(&s4);

    println!("The length of '{}' is {}", s4, len);

    // ? **************************************
    // ? Mutablel references
    // ? **************************************
    let mut s5 = String::from("hello s5");

    {
        //* begin of r1 scope
        let r1 = &mut s5;
        println!("r1: {}", r1);
    } //* end of r1 scope
    let r2 = &mut s5;

    //* r1 isn't avaiable here, but r2 is and for the rest of the main()
    println!("r2: {}", r2);

    //* references to a variable
    let reference_to_variable = no_dangle();
    println!("reference_to_variable: {}", reference_to_variable);

    // ? **************************************
    // ? String slices
    // ? **************************************
    let hello_world = String::from("hello world");
    let hello_world_literal = "helloagain world";

    let word = first_word(&hello_world);
    let word_literal = first_word(&hello_world_literal);

    println!("The first word is             : {}", word);
    println!("The first word of a litreal is: {}", word_literal);

}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn no_dangle() -> String {
    let s6 = String::from("hello s6");

    s6
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

fn calculate_length(s: &String) -> usize {
    s.len()
}
