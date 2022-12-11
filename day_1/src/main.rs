use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let filename = "aoc_input.txt".to_string();

    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut current_calories: i32 = 0;
    let mut highest_calories_number: [i32; 3] = [0, 0, 0];

    // Iterate over all lines
    for line in reader.lines() {
        // Check for reading error
        let current_line = match line {
            Ok(line) => line,
            Err(e) => return Err(e),
        };

        // Check if line is blank
        if current_line.is_empty() {            
            for i in 0..highest_calories_number.len() {

                if current_calories > highest_calories_number[i] {

                    if i != highest_calories_number.len() - 1 {
                        if current_calories > highest_calories_number[i + 1] {

                            continue;
                        }
                    }

                    highest_calories_number[i] = current_calories;
                }
            }

            current_calories = 0;

        } else {
            // Try to convert line to an int
            let number = match current_line.parse::<i32>() {
                Ok(number) => number,
                Err(_) => {
                    log::warn!("Converting line to number failed");
                    continue;
                },
            };
            current_calories += number;
        }
    }

    println!("The top {} carry in total:", highest_calories_number.len());
    let mut total_top_calories = 0;
    for c in highest_calories_number {
        total_top_calories += c;
    }
    println!("{} calories", total_top_calories);


    Ok(())
}
