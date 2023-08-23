use collection_exercises::ex1::{median::get_median, mode::get_mode};

fn main() {
    // *************************************************************************************
    // * Exercise 1
    // *************************************************************************************
    // Given a list of integers, use a vector and return the median (when sorted,
    // the value in the middle position) and
    // mode (the value that occurs most often;
    // a hash map will be helpful here) of the list.

    // let mut vec = vec![
    //     4, 9, 8, 7, 8, 9, 4, 5, 1, 3, 1, 2, 1, 6, 5, 4, 9, 8, 7, 6, 5, 1, 3, 2, 4, 9, 8, 7,
    // ];

    // println!("mode_map via function : {:?}", get_mode(&mut vec));

    // println!("median via function   : {:?}", get_median(&mut vec));

    // *************************************************************************************
    // * Exercise 2
    // *************************************************************************************
    // Convert strings to pig latin. The first consonant of each word is moved to the end of
    // the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel
    // have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details
    // about UTF-8 encoding!

    let vowel_vec = vec!["a", "e", "i", "o", "u"];
    let mut vowel_found = false;

    let test_string: String = "This is another string".to_string();
    let mut pig_latin_string = String::new();
    let words: Vec<&str> = test_string.split_whitespace().collect();

    for word in words {
        for letter in &vowel_vec {
            if &word[0..1] == *letter {
                vowel_found = true
            }
        }

        if vowel_found {
            let pig_latin = word[0..].to_string() + "-" + "hay";
            pig_latin_string.push_str(&(pig_latin.to_owned() + " "));
        } else {
            let pig_latin = word[1..].to_string() + "-" + &word[0..1].to_string() + "ay";
            pig_latin_string.push_str(&(pig_latin.to_owned() + " "));
        }
        // reset vowel flag
        vowel_found = false;
    }
    println!("test_string: {}", test_string);
    println!("pig_latin_string: {}", pig_latin_string);
}
