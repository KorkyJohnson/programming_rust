pub fn get_median(vec: &mut Vec<i32>) -> i32 {
    
    // sort the vector
    vec.sort();

    vec[vec.len() / 2]
}
