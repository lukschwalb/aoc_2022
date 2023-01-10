use std::collections::HashSet;

const PART_1_ROPE_LENGTH: u32 = 2;
const PART_2_ROPE_LENGTH: u32 = 10;

fn main() {
    let input = include_str!("../aoc_input.txt");

    let positions_part_1 = solve(input, PART_1_ROPE_LENGTH);
    println!("Positions visited for a rope with length of 2: {}", positions_part_1);

    let positions_part_2 = solve(input, PART_2_ROPE_LENGTH);
    println!("Positions visited for a rope with length of 10: {}", positions_part_2);
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new() -> Position {
        return Position {
            x: 0,
            y: 0,
        }
    }

    // Calculates the relative position of other position to self
    fn rel_x(&self, other_pos: &Position) -> i32 {
        return other_pos.x - self.x;
    }

    fn rel_y(&self, other_pos: &Position) -> i32 {
        return other_pos.y - self.y;
    }

    fn is_overstretched(&self, other_pos: &Position) -> bool {
        return (self.x - other_pos.x).abs() >= 2 ||
            (self.y - other_pos.y).abs() >= 2;
    }
}

enum CommandType {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

struct Command {
    cmd_type: CommandType,
    count: u32,
}

impl Command {
    fn from_str(string: &str) -> Option<Command> {
        let split: Vec<&str> = string.split(' ').collect();
        if split.len() >= 2 {
            let cmd_type = match split[0] {
                "U" => CommandType::UP,
                "D" => CommandType::DOWN,
                "L" => CommandType::LEFT,
                "R" => CommandType::RIGHT,
                _ => return None,
            };

            let count: u32 = split[1].parse().unwrap();
            return Some(Command {
                cmd_type: cmd_type,
                count: count,
            })
        }
        return None;
    }

    fn execute(&self, rope: &mut Vec<Position>, map: &mut HashSet<Position>) {
        for _i in 0..self.count {
            let old_rope = rope.clone();
            match self.cmd_type {
                CommandType::UP => rope[0].y += 1,
                CommandType::DOWN => rope[0].y -= 1,
                CommandType::LEFT => rope[0].x -= 1,
                CommandType::RIGHT => rope[0].x += 1,
            }

            if rope[0].is_overstretched(&rope[1]) {
                rope[1].x = old_rope[0].x;
                rope[1].y = old_rope[0].y;
        
                for i in 2..rope.len() {
                    if rope[i-1].is_overstretched(&rope[i]) {
                        let rel_x = rope[i-1].rel_x(&rope[i]);
                        let rel_y = rope[i-1].rel_y(&rope[i]);

                        if rel_x.abs() == 2 && rel_y.abs() == 2 {
                            rope[i].x = old_rope[i-1].x;
                            rope[i].y = old_rope[i-1].y;
                        } else if rel_x == 2 && rel_y == 0 {
                            rope[i].x -= 1;
                        } else if rel_x == -2 && rel_y == 0 {
                            rope[i].x += 1;
                        } else if rel_x == 0 && rel_y == 2 {
                            rope[i].y -= 1;
                        } else if rel_x == 0 && rel_y == -2 {
                            rope[i].y += 1;
                        } else if rel_x == -2 && rel_y == -1 { // Right - Up
                            rope[i].x += 1;
                            rope[i].y += 1;
                        } else if rel_x == -1 && rel_y == -2 { // Up - Right
                            rope[i].x += 1;
                            rope[i].y += 1;
                        } else if rel_x == 1 && rel_y == -2 { // Up - Left
                            rope[i].x -= 1;
                            rope[i].y += 1;
                        } else if rel_x == 2 && rel_y == -1 { // Left - Up
                            rope[i].x -= 1; 
                            rope[i].y += 1;
                        } else if rel_x == 2 && rel_y == 1 { // Left - Down
                            rope[i].x -= 1;
                            rope[i].y -= 1;
                        } else if rel_x == 1 && rel_y == 2 { // Down - Left
                            rope[i].x -= 1;
                            rope[i].y -= 1;
                        } else if rel_x == -2 && rel_y == 1 { // Right - Down
                            rope[i].x += 1;
                            rope[i].y -= 1;
                        } else if rel_x == -1 && rel_y == 2 { // Down - Right
                            rope[i].x += 1;
                            rope[i].y -= 1;
                        }
                    }
                }
            }
            map.insert(rope.last().unwrap().clone());
        }
    }
}

fn solve(input: &str, rope_length: u32) -> u32 {
    let mut map: HashSet<Position> = HashSet::new();
    let mut rope: Vec<Position> = (0..rope_length)
        .map(|_i| Position::new())
        .collect();

    input.lines()
        .map(|l| Command::from_str(l).unwrap())
        .for_each(|c| c.execute(&mut rope, &mut map));

    map.iter().len().try_into().unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT_1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const TEST_INPUT_2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_example_input_part_1() {
        let positions = solve(TEST_INPUT_1, PART_1_ROPE_LENGTH);
        assert_eq!(positions, 13)
    }

    #[test]
    fn test_example_input_part_2() {
        let positions = solve(TEST_INPUT_1, PART_2_ROPE_LENGTH);
        assert_eq!(positions, 1);

        let positions2 = solve(TEST_INPUT_2, PART_2_ROPE_LENGTH);
        assert_eq!(positions2, 36);
    }
}

/*
Right - Up                     Up - Right                   Left - Down            Right - Down
.......................................................................................................
........H.............................1H...............................................................
........1............................................................23................32..............
.....32..............................2.............................1......................1............
.....................................3.............................H......................H............
.......................................................................................................
-------------------------------------------------------------------------------------------------------
.......................................................................................................
.....H1............................H...............................3.........................3.........
...................................1...............................2.........................2.........
......2..............................23................................................................
......3..........................................................H1...........................1H.......
.......................................................................................................
  Up - Left                    Left - Up                       Down - Left               Down - Right */