
#[derive(Debug)]
pub struct Elf {
    snacks: Vec<u32>,
}

impl Elf {

    pub fn calories(&self) -> u32 {
        self.snacks.iter().sum()
    }
}

pub fn parse_input(input: &str) -> Vec<Elf> {
    input
        .split("\n\n")
        .map(|e| Elf { snacks: e.lines().filter_map(|l| l.parse::<u32>().ok()).collect() })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {

    parse_input(input).iter().map(|e| e.calories())
        .max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut elves = parse_input(input);
    elves.sort_by_cached_key(|e| e.calories());

    Some(elves.iter().rev().take(3).map(|e| e.calories()).sum())

}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
