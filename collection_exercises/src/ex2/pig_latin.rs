use regex::Regex;

pub fn to_pig_latin(user_string: String) -> String {
    let vowel_vec = vec!["a", "e", "i", "o", "u"];
    let mut vowel_found = false;
    let mut pig_latin_string = String::new();

    // Regex uppercase pattern
    let uppercase_regex_pattern = Regex::new(r"^\p{Lu}$").unwrap();

    // put all the words into a vector
    let user_words: Vec<&str> = user_string.split_whitespace().collect();

    for word in user_words {
        for vowel in &vowel_vec {
            if &word[0..1].to_ascii_lowercase() == *vowel {
                vowel_found = true
            }
        }

        if vowel_found {
            let pig_latin = word[0..].to_string() + "-" + "hay";
            pig_latin_string.push_str(&(pig_latin.to_owned() + " "));
        } else {
            // Its not a vowel so we have to move the word down one before adding it to the string
            let is_first_char_uppercase = uppercase_regex_pattern.is_match(&word[0..1].to_string());
            if is_first_char_uppercase {
                let first_char = &word[0..1].to_ascii_lowercase();
                let second_char = &word[1..2].to_ascii_uppercase();
                let new_word = second_char.to_owned() + &word[2..];
                let pig_latin = new_word + "-" + &first_char + "ay";
                pig_latin_string.push_str(&(pig_latin.to_owned() + " "));
            } else {
                let pig_latin = word[1..].to_string() + "-" + &word[0..1].to_string() + "ay";
                pig_latin_string.push_str(&(pig_latin.to_owned() + " "));
            }
        }
        // reset vowel flag
        vowel_found = false;
    }
    // return the sentence in pig lating
    pig_latin_string
}
