use std::collections::HashSet;

fn are_unique(data: &[char]) -> bool {
    let mut unique = HashSet::new();
    return data.iter().all(|x| unique.insert(x));
}

fn get_starting_index(data: &Vec<char>, window_size: usize) -> usize {
    return data
        .windows(window_size)
        .map(|w| are_unique(w))
        .position(|x| x)
        .unwrap() + window_size;
}

fn main() {
    let input = include_str!("../aoc_input.txt");
    let data = input.chars().collect();
    let starting_index_4 = get_starting_index(&data, 4);
    let starting_index_14 = get_starting_index(&data, 14);

    println!("The starting index with window size 4 is: {}", starting_index_4);
    println!("The starting index with window size 14 is: {}", starting_index_14);

}