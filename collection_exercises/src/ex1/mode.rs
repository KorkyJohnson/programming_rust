use std::collections::HashMap;

pub fn get_mode(vec: &mut Vec<i32>) -> HashMap<i32, i32> {
    // sort the vector
    vec.sort();

    let mut map: HashMap<&i32, i32> = HashMap::new();
    let mut mode_map: HashMap<i32, i32> = HashMap::new();
    let mut max_value = 0;

    // convert the vector to a hashmap
    for value in vec {
        let count = map.entry(value).or_insert(0);
        *count += 1;
    }

    // loop 1 - find the highest count out of all the Hashmap
    for (_key, value) in &map {
        if value > &max_value {
            max_value = *value;
        }
    }

    // loop 2 - find all the k:v that have the highest v and move them to the mode Hashmap
    for (key, value) in &map {
        if *value == max_value {
            mode_map.insert(**key, *value);
        }
    }

    mode_map // return the Hashmap w/ the mode
}
