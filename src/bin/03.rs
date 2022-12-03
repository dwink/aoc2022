use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    
    let mut result: u32 = 0;

    for line in input.lines() {

        let (left, right) = line.split_at(line.len() / 2);
        let lhs: HashSet<u8> = left.bytes().collect();
        let rhs: HashSet<u8> = right.bytes().collect();

        let priority = match *lhs.intersection(&rhs).next().unwrap() {
            n @ 65..=90 => n - 38,
            n @ 97..=122 => n - 96,
            _ => unreachable!()
        } as u32;

        result += priority;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {

    let in_vec = input.lines().collect::<Vec<&str>>();

    let mut result: u32 = 0;

    for chunk in in_vec.chunks(3) {
        let a: HashSet<u8> = chunk[0].bytes().collect();
        let b: HashSet<u8> = chunk[1].bytes().collect();
        let c: HashSet<u8> = chunk[2].bytes().collect();
        
        let priority = match a.intersection(&b).cloned().collect::<HashSet<u8>>().intersection(&c).next().unwrap() {
            n @ 65..=90 => n - 38,
            n @ 97..=122 => n - 96,
            _ => unreachable!()
        } as u32;

        result += priority;
    }

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
