fn main() {
    println!("********************************************************");
    println!("*** Chapter 8 - Collections ***");
    println!("********************************************************");
    println!("********************************************************");
    println!("* Vectors");
    println!("********************************************************");
    // let v: Vec<i32> = Vec::new();

    let mut v = vec![1, 2, 3];

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("2nd time the third element is {third}"),
        None => println!("There is no thrid element"),
    }

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![44, 55, 66];
    for i in &mut v {
        *i += 50;
        println!("{i}");
    }

    // you can vector w/ enums
    #[derive(Debug)]
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text("blue".to_string()),
        SpreadSheetCell::Float(4.89),
    ];

    let row2 = SpreadSheetCell::Text(("purple".to_string()));
    println!("Value of row2 is {:?}", row2);

    println!("********************************************************");
    println!("* Strings");
    println!("********************************************************");
    let s = String::new(); // empty String
    let t = "This is initialized".to_string(); // initialized string
    let u = String::from("This string is also instialized");

    let mut s1 = "foo".to_string();
    let s2 = "bar";
    s1.push_str(s2); // append s2 to s1 (foobar)
    println!("The value of s1 is now {s1}");

    let mut s = "lo".to_string();
    s.push_str("l"); // append to make lol
    println!("The value of s is now {s}");

    let hello = "hello".to_string();
    let world = "world".to_string();
    let hello_world0 = format!("{hello} {world}"); // need to create this here because we lose s1 in the next one
    let hello_world = hello + " " + &world; // can't do &hello + & world because are two references and you need at least one string (hello)
    println!("{hello_world}");
    println!("{hello_world0}");

    // Iterating over strings
    let ab = "ab".to_string();
    for i in ab.chars() {
        println!("{i}"); // displays UTF-8 chars
    }

    for i in ab.bytes() {
        println!("{i}"); // displays UTF-8 characters in bytes
    }
    println!("********************************************************");
    println!("* HashMaps");
    println!("********************************************************");

    use std::collections::HashMap; // need to do this because its not brought in automatically

    // let mut scores = HashMap::new();

    // scores.insert("Blue", 10);
    // scores.insert("Yellow", 50);

    // access values
    // let team_name = "Blue";
    // let score = scores.get(&team_name).copied().unwrap_or(0);

    // println!("score is {score}");

    // iterate through the Hashmap
    // for (key, value) in scores {
    //     println!("{key}, {value}");
    // }

    let field_name =  "Favourite Colour".to_string();
    let field_value = "Blue".to_string();
    
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    
    for (key, value) in map {
        println!("{key}, {value}");
    }

    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Blue", 25); // this one will be kept

    scores.entry("Blue").or_insert(50); // exists, value won't be inserted  nor current value replaced
    scores.entry("Yellow").or_insert(50); // doesn't exist, will be inserted

    println!("scores: {:?}", scores);

    let text = "hello world wonderful world";

    let mut map2 = HashMap::new();

    for word in text.split_whitespace() {
        let count = map2.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map2);
}
