fn main() {
    let input = include_str!("../aoc_input.txt");

    let strength_sum_part1 = solve(input);
    println!("Signal strength part 1: {}", strength_sum_part1);
}

#[derive(Eq, PartialEq)]
enum CPUState {
    BUSY,
    FREE,
}

fn render_crt(crt: &[[bool; 40]; 6]) {
    for row in 0..6 {
        for col in 0..40 {
            if crt[row][col] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn solve(input: &str) -> i32 {

    let mut cycle: i32 = 1;
    let mut x_reg: i32 = 1;
    let mut tmp_reg: i32 = 0;
    let mut state = CPUState::FREE;
    let mut signal_strengths: Vec<i32> = Vec::new();
    let mut crt = [[false; 40]; 6];
    let mut cmds: Vec<&str> = input.lines()
        .collect();
    cmds.reverse();

    while !cmds.is_empty() {

        // Calculate signal strength
        if (cycle + 20) % 40 == 0 {
            let strength = cycle * x_reg;
            signal_strengths.push(strength);
        }

        // Calculate the CRT
        let crt_pos = (cycle - 1) % 240;
        let crt_row = usize::try_from(crt_pos / 40).unwrap();
        let crt_col = usize::try_from(crt_pos % 40).unwrap();
        
        if crt_pos % 40 >= x_reg - 1 && crt_pos % 40 <= x_reg + 1 {
            crt[crt_row][crt_col] = true;
        } else {
            crt[crt_row][crt_col] = false;
        }

        // Start execution of command
        if state == CPUState::FREE {
            let cmd = cmds.pop().unwrap();

            if cmd.starts_with("noop") {

            } else if cmd.starts_with("addx") {
                tmp_reg = cmd.split_once(' ').unwrap().1.parse().unwrap();
                state = CPUState::BUSY;
            }
        } else {
            // addx finished, release CPU and add tmp_reg
            state = CPUState::FREE;
            x_reg += tmp_reg;
        }    
        cycle += 1;
    }

    render_crt(&crt);
    return signal_strengths.iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_part_1() {
        let test_input = include_str!("../test_input.txt");
        let strength_sum = solve(test_input);

        assert_eq!(strength_sum, 13140);
    }
}