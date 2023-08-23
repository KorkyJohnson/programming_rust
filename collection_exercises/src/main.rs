use collection_exercises::ex1::{median::get_median, mode::get_mode};

fn main() {
    // Given a list of integers, use a vector and return the median (when sorted,
    // the value in the middle position) and
    // mode (the value that occurs most often;
    // a hash map will be helpful here) of the list.

    let mut vec = vec![
        4, 9, 8, 7, 8, 9, 4, 5, 1, 3, 1, 2, 1, 6, 5, 4, 9, 8, 7, 6, 5, 1, 3, 2, 4, 9, 8, 7,
    ];

    println!("mode_map via function : {:?}", get_mode(&mut vec));

    println!("median via function   : {:?}", get_median(&mut vec));
}
