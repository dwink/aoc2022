use std::convert::TryFrom;

// A: Rock (1)
// B: Paper (2) 
// C: Scissors (3)
//
// X: Rock
// Y: Paper
// Z: Scissors

// Scissors beats Paper
// Rock beats Scissors
// Paper beats Rock

// Lose: 0
// Draw: 3
// Win: 6

#[derive(PartialEq)]
enum Choice {
    Rock = 1,
    Paper,
    Scissors
}

impl TryFrom<char> for Choice {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Choice::Rock),
            'B' | 'Y' => Ok(Choice::Paper),
            'C' | 'Z' => Ok(Choice::Scissors),
            _ => Err("Unknown RPS value")
        }
    }
}

impl Choice {

    fn win(&self) -> Choice {
        match self {
            Choice::Scissors => Choice::Rock,
            Choice::Paper => Choice::Scissors,
            Choice::Rock => Choice::Paper
        }
    }

    fn lose(&self) -> Choice {
        match self {
            Choice::Scissors => Choice::Paper,
            Choice::Paper => Choice::Rock,
            Choice::Rock => Choice::Scissors
        }
    }
}

#[derive(PartialEq)]
enum GameResult {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl TryFrom<char> for GameResult {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(GameResult::Loss),
            'Y' => Ok(GameResult::Draw),
            'Z' => Ok(GameResult::Win),
            _ => Err("Unknown Result value")
        }
    }
}


pub fn part_one(input: &str) -> Option<u32> {

    let mut result: u32 = 0;
    for line in input.lines() {

        if let Some((opp, ply)) = line.split_once(' ') {

            let opp = Choice::try_from(opp.chars().next().unwrap()).unwrap();
            let ply = Choice::try_from(ply.chars().next().unwrap()).unwrap();

            let score = match (opp, ply) {
                (Choice::Rock, b @ Choice::Paper) |
                (Choice::Paper, b @ Choice::Scissors) |
                (Choice::Scissors, b @ Choice::Rock) => 6u8 + b as u8,
                (a, b) if a == b => 3u8 + b as u8,
                (_, b) => b as u8
            };

            result += score as u32;

        }
    }
    Some(result) 
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    for line in input.lines() {

        if let Some((opp, res)) = line.split_once(' ') {

            let opp = Choice::try_from(opp.chars().next().unwrap()).unwrap();
            let res = GameResult::try_from(res.chars().next().unwrap()).unwrap();

            let score = match (opp, res) {
                (a, b @ GameResult::Draw) => b as u8 + a as u8,
                (a, b @ GameResult::Win) => b as u8 + a.win() as u8,
                (a, b @ GameResult::Loss) => b as u8 + a.lose() as u8,
            };

            result += score as u32;

        }
    }
    Some(result) 
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
