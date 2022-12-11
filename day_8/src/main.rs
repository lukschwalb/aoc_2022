use std::fs::File;
use std::io::{self, prelude::*, BufReader};


fn check_tree(map: &Vec<Vec<u32>>, check_x: usize, check_y: usize) -> (bool, u32) {
    let map_width = map[0].len();
    let map_height = map.len();
    let tree_height = map[check_y][check_x];

    let axis: [char; 4] = ['y', 'y', 'x', 'x'];
    let mut ranges: Vec<Vec<usize>> = Vec::new();
    ranges.push((0..check_y).rev().collect()); // Top
    ranges.push((check_y+1..map_height).collect()); // Bottom
    ranges.push((0..check_x).rev().collect()); // Left
    ranges.push((check_x+1..map_width).collect()); // Right

    let mut visibility: [bool; 4] = [true, true, true, true];
    let mut tree_score = 1;

    for dir in 0..4 {
        let mut visible_count = 0;
        let mut current_max_height = 0;
        for v in &ranges[dir] {
            let mut height = 0;

            if axis[dir] == 'y' {
                height = map[*v][check_x];
            } else if axis[dir] == 'x' {
                height = map[check_y][*v];
            }

            if current_max_height < tree_height{
                visible_count += 1;
                current_max_height = height;
            }
            if height >= tree_height {
                visibility[dir] = false;
            }
        }
        tree_score *= visible_count;

    }

    let visible = visibility[0] || visibility[1] || visibility[2] || visibility[3];
    return (visible, tree_score);
}

fn print_map(map: Vec<Vec<bool>>) {
    for row in map {
        for item in row {
            print!("{}", if item {1} else {0});
        }
        print!{"\n"};
    }
}

fn main() -> io::Result<()> {
    let filename = "aoc_input.txt".to_string();
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut tree_map: Vec<Vec<u32>> = Vec::new();

    let mut visible_trees = 0;
    let mut max_tree_score = 0;

    for line in reader.lines() {
        let mut tree_line: Vec<u32> = Vec::new();
        let tree_line_string = line?;

        for tree_char in tree_line_string.chars() {
            tree_line.push(tree_char.to_digit(10).unwrap());
        }
        tree_map.push(tree_line);
    }

    let map_width = tree_map[0].len();
    let map_height = tree_map.len();
    let mut visible_map: Vec<Vec<bool>> = vec![vec![Default::default(); map_width]; map_height];
    
    for y in 0..map_height {
        for x in 0..map_width {
            let (is_visible, tree_score) = check_tree(&tree_map, x, y);
            
            if is_visible { 
                visible_trees += 1; 
                visible_map[y][x] = true;
            }

            if tree_score > max_tree_score { 
                max_tree_score = tree_score; 
            }
        }
    }

    print_map(visible_map);
    println!("Visible trees: {}", visible_trees);
    println!("Maximum tree score: {}", max_tree_score);

    Ok(())
}
