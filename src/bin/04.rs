use itertools::Itertools;
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {

    let result = input
        .lines()
        .map(
            |l| l
            .split(',')
            .map(
                |m| m
                .split('-')
                .map(
                    |i| i
                    .parse::<u32>().unwrap())
                .collect_tuple().unwrap())
            .collect::<Vec<(u32, u32)>>()
            )
        .filter(
            |a: &Vec<(u32, u32)>| ((a[0].0 <= a[1].0) && (a[0].1 >= a[1].1)) || ((a[1].0 <= a[0].0) && (a[1].1 >= a[0].1))  )
        .count() as u32;

    Some(result)
}

fn overlap(a: (u32, u32), b: (u32, u32)) -> bool {
    let left: HashSet<u32> = (a.0..=a.1).collect();
    let right: HashSet<u32> = (b.0..=b.1).collect();

    left.intersection(&right).count() > 0
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(
            |l| l
            .split(',')
            .map(
                |m| m
                .split('-')
                .map(
                    |i| i
                    .parse::<u32>().unwrap())
                .collect_tuple().unwrap())
            .collect::<Vec<(u32, u32)>>()
            )
        .filter(
            |a: &Vec<(u32, u32)>| overlap(a[0], a[1]))
        .count() as u32;

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
