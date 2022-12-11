use std::fs::File;
use std::io::{self, prelude::*, BufReader};


fn check_for_containment(   start_sector_1: u32, 
                            end_sector_1: u32, 
                            start_sector_2: u32,
                            end_sector_2: u32) -> bool {
    // Check if sector 2 fully contains sector 1
    if start_sector_1 >= start_sector_2 && end_sector_1 <= end_sector_2 {
        return true;
    // Check if sector 1 fully contains sector 2
    } else if start_sector_2 >= start_sector_1 && end_sector_2 <= end_sector_1  {
        return true;
    }

    return false;
}

fn check_for_overlap(   start_sector_1: u32, 
                        end_sector_1: u32, 
                        start_sector_2: u32,
                        end_sector_2: u32) -> bool {
    // Check if sector 1 overlaps sector 2
    if start_sector_1 <= start_sector_2 && end_sector_1 >= start_sector_2 {
        return true;
    // Check if sector 2 overlaps sector 1
    } else if start_sector_2 <= start_sector_1 && end_sector_2 >= start_sector_1  {
        return true;
    }

    return false;
}

fn main() -> io::Result<()> {
    let filename = "aoc_input.txt".to_string();

    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut containments = 0;
    let mut overlaps = 0;

    for line in reader.lines() {
        let binded = line?;
        let split_comma = binded.split(",");
        let mut sections: [u32; 4] = Default::default();
        let mut current_section: usize = 0;

        for elf_sectors in split_comma {
            let mut split_dash = elf_sectors.split("-");

            sections[current_section * 2] = split_dash.next().unwrap().parse::<u32>().unwrap();
            sections[current_section * 2 + 1] = split_dash.next().unwrap().parse::<u32>().unwrap();
            current_section += 1;
        }

        if check_for_containment(sections[0], sections[1], sections[2], sections[3]) {
            containments += 1;
        }

        if check_for_overlap(sections[0], sections[1], sections[2], sections[3]) {
            overlaps += 1;
        }
    }

    println!("Number of containments: {}", containments);
    println!("Number of overlaps: {}", overlaps);
    Ok(())
}
