use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use simple_error::SimpleError;

#[derive(Copy, Clone, PartialEq)]
enum RPSHand {
    // (wins against, loses against)
    Rock,
    Paper,
    Scissors,
}

#[derive(Copy, Clone, PartialEq)]
enum RPSResult {
    Win,
    Loss,
    Draw,
}

impl RPSHand {
    fn from(chr: char) -> Result<RPSHand, SimpleError> {
        match chr {
            'X' => Ok(RPSHand::Rock),
            'Y' => Ok(RPSHand::Paper),
            'Z' => Ok(RPSHand::Scissors),
            'A' => Ok(RPSHand::Rock),
            'B' => Ok(RPSHand::Paper),
            'C' => Ok(RPSHand::Scissors),
            _ => Err(SimpleError::new("String does not match any RPSHand pattern")),
        }
    }

    fn score(&self) -> i32 {
        match self {
            RPSHand::Rock => 1,
            RPSHand::Paper => 2,
            RPSHand::Scissors => 3,
        }
    }

    fn get_win(&self) -> RPSHand {
        match self {
            RPSHand::Rock => RPSHand::Scissors,
            RPSHand::Paper => RPSHand::Rock,
            RPSHand::Scissors => RPSHand::Paper,
        }
    }

    fn get_loss(&self) -> RPSHand {
        match self {
            RPSHand::Rock => RPSHand::Paper,
            RPSHand::Paper => RPSHand::Scissors,
            RPSHand::Scissors => RPSHand::Rock,
        }
    }
}

impl RPSResult {
    fn from(chr: char) -> Result<RPSResult, SimpleError> {
        match chr {
            'X' => Ok(RPSResult::Loss),
            'Y' => Ok(RPSResult::Draw),
            'Z' => Ok(RPSResult::Win),
            _ => Err(SimpleError::new("String does not match any RPSResult pattern")),
        }
    }

    fn score(&self) -> i32 {
        match self {
            RPSResult::Win => 6,
            RPSResult::Loss => 0,
            RPSResult::Draw => 3,
        }
    }
}

fn evaluate_game(own_hand: RPSHand, opp_hand: RPSHand) -> RPSResult {
    if own_hand == opp_hand {
        return RPSResult::Draw;
    } else if opp_hand == own_hand.get_win() {
        return RPSResult::Win;
    } else {
        return RPSResult::Loss;
    }
}

fn determine_own_hand(opp_hand: RPSHand, result: RPSResult) -> RPSHand {
    if result == RPSResult::Draw {
        return opp_hand;
    } else if result == RPSResult::Win {
        return opp_hand.get_loss();
    } else {
        return opp_hand.get_win();
    }
}

fn main() -> io::Result<()> {
    let filename = "aoc_input.txt".to_string();

    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut total_score_part1 = 0;
    let mut total_score_part2 = 0;

    for line_result in reader.lines() {
        let line = line_result?;

        // Check what hand the other player wants to play
        if let Ok(other_hand) = RPSHand::from(line.chars().nth(0).unwrap()) {

            // Check what we want to play in case of part1
            if let Ok(self_hand) = RPSHand::from(line.chars().nth(2).unwrap()) {
                let result = evaluate_game(self_hand, other_hand);
        
                total_score_part1 += self_hand.score() + result.score();
            }

            // Check the intended game result in case of part2
            if let Ok(result) = RPSResult::from(line.chars().nth(2).unwrap()) {
                let self_hand = determine_own_hand(other_hand, result);
        
                total_score_part2 += self_hand.score() + result.score();
            }

        }
    }

    println!("Total score for part 1 is {}", total_score_part1);
    println!("Total score for part 2 is {}", total_score_part2);

    Ok(())
}