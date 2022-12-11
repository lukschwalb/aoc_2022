use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn find_common_chars(first: &str, second: &str) -> Vec<char> {
    let mut found_chars: Vec<char> = Vec::new();

    for first_char in first.chars() {
        for second_char in second.chars() {
            if first_char == second_char {
                if !found_chars.contains(&first_char) {
                    found_chars.push(first_char)
                }
            }
        }
    }

    return found_chars;
}

fn get_priority(chr: char) -> u32 {
    return match chr {
        'a'..='z' => chr as u32 - 96,
        'A'..='Z' => chr as u32 - 38,
        _ => 0, 
    }
}

fn main() -> io::Result<()> {
    let filename = "aoc_input.txt".to_string();

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut elf_groups: [String; 3] = Default::default();
    let mut priority_sum_part1 = 0;
    let mut priority_sum_part2 = 0;
    let mut group_counter: usize = 0;

    for line in reader.lines() {
        let pack_list = line?;
        let length = pack_list.chars().count();
        let common = find_common_chars(&pack_list[..length/2], &pack_list[length/2..]);
        priority_sum_part1 += get_priority(common[0]);

        elf_groups[group_counter] = pack_list.clone();

        if group_counter == 2 {
            let common_chars1: String = find_common_chars(&elf_groups[0], &elf_groups[1]).into_iter().collect();
            println!("{}", elf_groups[1]);
            let common_chars2 = find_common_chars(&common_chars1, &elf_groups[2]);

            if common_chars2.len() > 0 {
                priority_sum_part2 += get_priority(common_chars2[0]);
            }
            group_counter = 0
        } else {
            group_counter += 1;
        }
    }
    println!("Sum of priorities for part 1 is: {}", priority_sum_part1);
    println!("Sum of priorities for part 2 is: {}", priority_sum_part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STRING: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_find_common_type() {
        let common_types: [char; 6] = ['p', 'L', 'P', 'v', 't', 's'];
        let binding = TEST_STRING;
        let split = binding.split('\n');
        
        let mut i = 0;
        for line in split {
            let length = line.chars().count();
            let common = find_common_type(&line[..length/2-1], &line[length/2..]);
            assert_eq!(common, common_types[i]);
            i += 1;
        }
    }

    #[test]
    fn test_get_priority() {
        let test_chars: [char; 4] = ['a', 'z', 'A', 'Z'];
        let test_priorities: [u32; 4] = [1, 26, 27, 52];
        
        for i in 0..4 {
            let prio = get_priority(test_chars[i]);
            assert_eq!(prio, test_priorities[i]);
        }
    }

    #[test]
    fn full_test() {
        let binding = TEST_STRING;
        let split = binding.split('\n');
        
        let mut sum = 0;
        for line in split {
            let length = line.chars().count();
            let common = find_common_type(&line[..length/2-1], &line[length/2..]);
            let prio = get_priority(common);
            sum += prio;
        }

        assert_eq!(sum, 157);
    }
}