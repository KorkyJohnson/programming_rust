use std::collections::HashMap;
fn main() {

    // Given a list of integers, use a vector and return the median (when sorted,
    // the value in the middle position) and
    // mode (the value that occurs most often;
    // a hash map will be helpful here) of the list.

    let mut vec = vec![
        4, 9, 8, 7, 8, 9, 4, 5, 1, 3, 1, 2, 1, 6, 5, 4, 9, 8, 7, 6, 5, 1, 3, 2, 4, 9, 8, 7,
    ];

    // sort the vector
    vec.sort();

    let mut map: HashMap<&i32, i32> = HashMap::new();

    // convert the vector to a hashmap
    for value in &vec {
        let count = map.entry(value).or_insert(0);
        *count += 1;
    }

    let mut mode_map: HashMap<i32, i32> = HashMap::new();
    let mut max_value = 0;

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

    println!("mode_map {:?}", mode_map);

    let mid = &vec[&vec.len() / 2];

    println!("median: {:?}", mid);
    
}