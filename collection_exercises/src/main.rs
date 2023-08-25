use collection_exercises::ex1::{median::get_median, mode::get_mode};
use collection_exercises::ex2::pig_latin::to_pig_latin;
use std::io;

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
    let mut user_string = String::new();

    println!("Enter a sentence and I will transform it into pig latin:");
    io::stdin()
        .read_line(&mut user_string)
        .expect("You didn't enter anything");

    println!(
        "Your sentence in pig latin: {:?}",
        to_pig_latin(user_string)
    );
}
