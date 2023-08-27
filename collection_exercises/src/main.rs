// use collection_exercises::ex1::{median::get_median, mode::get_mode};
// use collection_exercises::ex2::pig_latin::to_pig_latin;
use collection_exercises::ex3::menu::display_menu;
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
    // let mut user_string = String::new();

    // println!("Enter a sentence and I will transform it into pig latin:");
    // io::stdin()
    //     .read_line(&mut user_string)
    //     .expect("You didn't enter anything");

    // println!(
    //     "Your sentence in pig latin: {:?}",
    //     to_pig_latin(user_string)
    // );

    // *************************************************************************************
    // * Exercise 3
    // *************************************************************************************
    // Using a hash map and vectors, create a text interface to allow a user to add employee
    // names to a department in a company. For example, “Add Sally to Engineering” or
    // “Add Amir to Sales.” Then let the user retrieve a list of all people in a department
    // or all people in the company by department, sorted alphabetically.

    // Step 1   - Need to request user input
    //          - Add or Disp people
    //          - parse 'Add Bob to Programmers
    //          - Bob is name (key) / Programmers is job (value)
    //          - need to split
    //          - move all words to vector?
    // Step 2   - Need to move K/V to hashmap
    // Step 3   - When display, then show list of users alphaBetically
    //          - If none, then display 'Noone has been added"
    // Bonus    - Allow user to subtract people from the hashmap as well

    display_menu();

    loop {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("You didn't enter anything");

        if let Some(user_choice) = user_input.trim().chars().next() {
            match user_choice.to_ascii_lowercase() {
                'a' => println!("User wants to add"),
                'p' => println!("User wants to print"),
                'd' => display_menu(),
                'q' => {
                    println!("Goodbye :)");
                    break;
                }
                _ => println!("Please make a valid selection, press 'D' to display the menu"),
            }
        } else {
            println!("No valid input entered");
        }
    }
}
