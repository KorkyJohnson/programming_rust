fn main() {
    let mut found_end = false;
    let hello_world = String::from("hello world");
    let hello_world_len = hello_world.len();
    let chars = 0;

    while chars < hello_world_len {
        let word = first_word(&hello_world);
        println!("The first word of '{}' is {}", hello_world, word);
        found_end = true;
    }
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
